/**
 * npct的 运行时 配置
 */
pub struct Config<'a> {
    from_path: &'a str,
    module: Module,
    input: &'a str,
    output: &'a str,
}

impl Config<'_> {
    /**
     * 使用cli接受到的参数创建配置
     */
    fn from_args(args: &Vec<&String>) -> Self {
        Config {
            from_path: "",
            module: Module::FileServer(vec![]),
            input: "",
            output: "",
        }
    }
}


/**
 * npct 包含的功能模块
 */
pub enum Module {
    FileServer(Vec<Func>), RestfulServer(Vec<Func>),
}
/**
 * 对应于命令行中的 简写、全拼参数，如：
 * --version 或者 -v 版本
 * 
 */
pub enum Func {
    Version,
}

/**
 * tests
 */
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("it works!!!");
    }
}