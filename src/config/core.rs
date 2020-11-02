use std::env;
/**
 * npct的 运行时 配置
 */
pub struct Config {
    pub from_path: String,
    pub module: Module,
    pub input: String,
    pub output: String,
}

impl Config {
    /**
     * 使用cli接受到的参数创建配置
     */
    pub fn from_args(mut args: env::Args) -> Result<Self, &'static str> {
        let from_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Can't get execute path"),
        };

        let module = match args.next() {
            Some(arg) => arg,
            None => return Err("Can't get function"),
        };

        let module = match module.as_str() {
            "--version" => Func::Version,
            "-v" => Func::Version,
            _ => return Err("Invalid function"),
        };

        Ok(
            Config {
                from_path,
                module: Module::FileServer(vec![]),
                input: String::from(""),
                output: String::from(""),
            }
        )
    }
}


/**
 * npct 包含的功能模块
 */
pub enum Module {
    FileServer(Vec<Func>), RestfulMockServer(Vec<Func>),
}
/**
 * 对应于命令行中的 简写、全拼参数，如：
 * --version 或者 -v 版本
 * 
 */
pub enum Func {
    Version, Help
}