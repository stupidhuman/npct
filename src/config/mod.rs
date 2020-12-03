use std::env;
/**
 * 用于解析命令行输入的模块
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
     * 命令示例：
     *      npct file-server --path=..\file\image\
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

        let mut module = match module.as_str() {
            "file-server" => Module::FileServer,
            "restful-mock-server" => Module::RestfulMockServer,
            "help" => Module::Help,
            _ => return Err("Invalid module"),
        };

        let func: Vec<Func> = Vec::new();

        Ok(
            Config {
                from_path,
                module,
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
    FileServer, RestfulMockServer, Help,
}
/**
 * 对应于命令行中的 简写、全拼参数，如：
 * --version 或者 -v 版本
 *
 */
pub enum Func {
    Version, Help
}

