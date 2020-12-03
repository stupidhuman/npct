/**
 * mock-server模块（用于 创建 虚拟接口）
 */
pub struct ServerConfig {
    pub func: Func,
}

pub enum Func {
    Help,
    Version,
    Route,
}

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
        println!("it works!");
    }
}