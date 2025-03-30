use std::{any::Any, sync::Arc};

use anyhow::{Context, anyhow};
use async_trait::async_trait;
use dashmap::DashMap;

type Result<T> = std::result::Result<T, anyhow::Error>;

// StepAction 和 Step trait 保持不变
#[derive(Debug, Clone, Copy, PartialEq, Eq, strum_macros::Display)]
pub enum StepAction {
    #[strum(serialize = "ActionContinue")]
    Continue,
    #[strum(serialize = "ActionHalt")]
    Halt,
}

#[async_trait]
pub trait Step: Send + Sync {
    async fn run(&self, state: impl StateBag + 'async_trait) -> Result<StepAction>;
    async fn cleanup(&self, state: impl StateBag + 'async_trait) -> Result<()>;
}

#[derive(Debug, Default)]
pub struct StepNoop;

#[async_trait]
impl Step for StepNoop {
    async fn run(&self, _state: impl StateBag) -> Result<StepAction> {
        Ok(StepAction::Continue)
    }

    async fn cleanup(&self, _state: impl StateBag) -> Result<()> {
        Ok(())
    }
}

pub trait Config {
    fn prepare(&mut self) -> Result<()>;
}

pub type StateBagValue = Arc<dyn Any + Send + Sync>;

pub trait StateBag: Send + Sync {
    fn contains(&self, key: &str) -> bool;
    fn get(&self, key: &str) -> Result<StateBagValue>;
    fn update(&self, key: &str, new_value: StateBagValue) -> Result<()>;
    fn insert(&self, key: &str, value: StateBagValue) -> Result<()>;
    fn remove(&self, key: &str) -> Result<()>;
}

/// 真正的共享状态实现
#[derive(Debug, Default)]
pub struct SharedStateBag {
    data: DashMap<String, StateBagValue>,
}

impl SharedStateBag {
    /// 创建一个新的可克隆包装器
    pub fn into_shared(self) -> SharedStateBagHandle {
        SharedStateBagHandle {
            inner: Arc::new(self),
        }
    }
}

impl StateBag for SharedStateBag {
    fn contains(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    fn get(&self, key: &str) -> Result<StateBagValue> {
        self.data
            .get(key)
            .map(|v| Arc::clone(v.value()))
            .with_context(|| format!("Key '{}' not found", key))
    }

    fn update(&self, key: &str, new_value: StateBagValue) -> Result<()> {
        let mut entry = self
            .data
            .get_mut(key)
            .with_context(|| format!("Key '{}' not found", key))?;
        *entry.value_mut() = new_value;
        Ok(())
    }

    fn insert(&self, key: &str, value: StateBagValue) -> Result<()> {
        if self.data.contains_key(key) {
            return Err(anyhow!("Key '{}' already exists", key));
        }
        self.data.insert(key.to_string(), value);
        Ok(())
    }

    fn remove(&self, key: &str) -> Result<()> {
        self.data
            .remove(key)
            .map(|_| ())
            .with_context(|| format!("Key '{}' not found", key))
    }
}

/// 可克隆的共享状态句柄
#[derive(Debug, Clone)]
pub struct SharedStateBagHandle {
    inner: Arc<SharedStateBag>,
}

impl StateBag for SharedStateBagHandle {
    fn contains(&self, key: &str) -> bool {
        self.inner.contains(key)
    }

    fn get(&self, key: &str) -> Result<StateBagValue> {
        self.inner.get(key)
    }

    fn update(&self, key: &str, new_value: StateBagValue) -> Result<()> {
        self.inner.update(key, new_value)
    }

    fn insert(&self, key: &str, value: StateBagValue) -> Result<()> {
        self.inner.insert(key, value)
    }

    fn remove(&self, key: &str) -> Result<()> {
        self.inner.remove(key)
    }
}

// 使用示例
#[tokio::main]
async fn main() -> Result<()> {
    // 创建共享状态
    let shared_state = SharedStateBag::default().into_shared();

    // 插入一些数据
    shared_state.insert("counter", Arc::new(110usize))?;

    // 创建步骤
    let step = StepNoop;

    // 运行步骤（克隆共享状态句柄）
    step.run(shared_state.clone()).await?;

    // 可以继续在其他地方使用同一个状态
    let counter = shared_state
        .get("counter")?
        .downcast_ref::<usize>()
        .cloned()
        .unwrap();
    println!("Counter: {}", counter);

    Ok(())
}
