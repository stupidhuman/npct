//! 用于解析命令行参数的模块
//! # 命令行Example
//! npct [功能模块名] [附加参数（"-"开头的简写参数/"--"开头的全拼参数）] [模块输入参数1] [模块输入参数2] ....
use std::env;

pub struct InitialConfig {
    pub from_path: String,
    pub module: String,
    pub input: String,
    pub output: String,
}

impl InitialConfig {
    /**
     * 使用cli接受到的参数创建配置
     * 命令示例：
     *      npct file-server --path=..\file\image\
     */
    pub fn from_args(mut args: env::Args) -> Result<Self, &'static str> {
        Err("")
    }
}

/**
 * 模块的功能
 */
pub enum Func {
    Version,
    Help,
}

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
        println!("it works!");
    }
}

