use std::env;
/**
 * 用于解析命令行输入的模块
 */
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

