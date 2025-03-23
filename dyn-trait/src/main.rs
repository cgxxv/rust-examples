#![allow(unused)]

use std::{any::Any, sync::Arc};

use async_trait::async_trait;

pub type Result<T> = std::result::Result<T, Box<dyn Any + Send + Sync>>;

#[async_trait]
pub trait Step: Send + Sync {
    async fn run(&self, state: &dyn StateBag) -> Result<()>;
    async fn cleanup(&self, state: &dyn StateBag) -> Result<()>;
}

pub trait StateBag: Send + Sync {
    fn contains(&self, key: &str) -> bool;
}

pub type StateBagValue = Arc<dyn Any + Send + Sync>;

#[derive(Debug, Default)]
pub struct BasicStateBag;

impl StateBag for BasicStateBag {
    fn contains(&self, key: &str) -> bool {
        unimplemented!()
    }
}

pub struct StepHello {
    name: String,
}

impl StepHello {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

#[async_trait]
impl Step for StepHello {
    async fn run(&self, state: &dyn StateBag) -> Result<()> {
        println!("=> run: Hello, {}!", self.name);
        Ok(())
    }

    async fn cleanup(&self, state: &dyn StateBag) -> Result<()> {
        println!("=> cleanup: Hello, {}!", self.name);
        Ok(())
    }
}

pub struct StepWorld {
    name: String,
}

impl StepWorld {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

#[async_trait]
impl Step for StepWorld {
    async fn run(&self, state: &dyn StateBag) -> Result<()> {
        println!("=> run: World, {}!", self.name);
        Ok(())
    }

    async fn cleanup(&self, state: &dyn StateBag) -> Result<()> {
        println!("=> cleanup: World, {}!", self.name);
        Ok(())
    }
}

pub struct StepMulti {
    steps: Vec<Box<dyn Step>>,
}

impl StepMulti {
    pub fn new(steps: Vec<Box<dyn Step>>) -> Self {
        Self { steps }
    }
}

#[async_trait]
impl Step for StepMulti {
    async fn run(&self, state: &dyn StateBag) -> Result<()> {
        for step in &self.steps {
            step.run(state).await?;
        }
        Ok(())
    }

    async fn cleanup(&self, state: &dyn StateBag) -> Result<()> {
        for step in &self.steps {
            step.cleanup(state).await?;
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let steps: Vec<Box<dyn Step>> = vec![
        Box::new(StepHello::new("Step 1")),
        Box::new(StepWorld::new("Step 2")),
    ];

    let state = BasicStateBag;
    let runner = StepMulti::new(steps);

    runner.run(&state).await.unwrap();
}
