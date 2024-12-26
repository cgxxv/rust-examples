use std::env;

#[derive(Debug)]
pub struct Config {
    pub file: String,
    pub filter: String,
    pub target: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let file = match args.next() {
            Some(arg) => arg,
            None => return Err("请输入文件路径"),
        };

        let filter = match args.next() {
            Some(arg) => arg,
            None => return Err("请输入要计算的列"),
        };

        let target = match args.next() {
            Some(arg) => arg,
            None => return Err("请输入要保存的列"),
        };

        Ok(Config {
            file,
            filter,
            target,
        })
    }
}
