use std::{collections::HashSet, str::FromStr};

use macros::{register_task, task_param};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

#[task_param]
pub struct MelonParam {
    #[validate(email)]
    #[schemars(extend("x-priority" = 1))]
    pub user_name: String,

    #[validate(email)]
    #[schemars(extend("x-priority" = 500))]
    pub pay_email: String,
    pub product_id: String,
    pub pay_type: String,
    // #[schemars(with = "String")]
    pub auto_pay: Option<String>,
    pub schedule_no: String,
    #[schemars(with = "String")]
    pub transfer_owner: Option<String>,
    pub mode: String,
    pub before_second: u64,
    #[cfg(not(any(env = "product", env = "beta")))]
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub queue_count: u64,
    #[cfg(not(any(env = "product", env = "beta")))]
    #[schemars(extend("x-priority" = 900))]
    pub send_restock: String,
}

#[register_task(name = "melon_ticket", website = "Melon Ticket", task_type = TaskType::Aco, MelonParam, product)]
pub struct MelonTask {
    pub param: MelonParam,
    pub inicis_domain: String,
}

impl MelonTask {
    pub async fn new_tasks_with_values(_values: Vec<Value>) -> Result<()> {
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct TaskDescription {
    // task name and task path
    pub task_name: &'static str,
    // create task param
    pub fields: fn() -> Vec<String>,
    pub task_type: TaskType,
    // pub new_task_fn: NewTaskFn,
    // pub resources: fn() -> Vec<Resource>,
    pub website: &'static str,
    pub json_schema: fn() -> schemars::Schema,
    #[allow(dead_code)]
    pub level: TaskLevel,
    // pub param_fixes: Option<&'static phf::Map<&'static str, &'static str>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TaskLevel {
    Dev,
    Product,
}

#[derive(Debug, Clone, Eq, PartialEq, strum_macros::EnumIter, strum_macros::Display)]
pub enum TaskType {
    Waiting,
    Queue, //排队任务
    Aco,   //自动结账
    Login,
    Register,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct Keywords {
    pub raw: String,
    pub keywords: Vec<String>,
    pub accurate_keywords: Vec<String>,
    pub number_keywords: HashSet<i64>,
    pub negative_keywords: Vec<String>,
    pub relative_number_keywords: Vec<(i64, i64)>,
}

impl Keywords {
    pub fn new(keywords_string: String) -> Self {
        unimplemented!()
    }
}

pub fn deserialize_string_to_number<'de, D, T>(deserializer: D) -> std::result::Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr + Deserialize<'de>,
    <T as FromStr>::Err: std::fmt::Display,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrInt<T> {
        String(String),
        Number(T),
    }

    match StringOrInt::<T>::deserialize(deserializer)? {
        StringOrInt::String(s) => {
            let s = s.trim();
            s.parse::<T>().map_err(|e| {
                serde::de::Error::custom(format!("{}, {} can not parse to number", e, s))
            })
        }
        StringOrInt::Number(i) => Ok(i),
    }
}

pub fn deserialize_keyword_from_string<'de, D>(
    deserializer: D,
) -> std::result::Result<Keywords, D::Error>
where
    D: Deserializer<'de>,
{
    match Value::deserialize(deserializer)? {
        Value::String(s) => Ok(Keywords::new(s)),
        Value::Null => Ok(Keywords::default()),
        _ => Err(serde::de::Error::custom(
            "field format error, can not parsed to keywords",
        )),
    }
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Sync + Send>>;

fn main() {
    println!("Hello, world!");

    let ordered_fields = MelonParam::get_ordered_fields();
    println!("=> {ordered_fields:#?}");

    let fields_with_priority = MelonParam::get_fields_with_priority();
    println!("=> {fields_with_priority:#?}");
}
