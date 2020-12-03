/**
 * file-server模块（用于对 本地文件/网络文件 进行静态路由）
 */
pub struct FileServer {
    pub func: Func,
}

pub struct RouteConfig {
    // 需要被路由的文件（夹）路径
    dir: &'static str,
    // 是否启用缓存（具体机制待设计）
    cache: bool,
    // 外部访问端口
    port: u32,
}

pub enum Func {
    // 功能-获取帮助
    Help,
    // 功能-获取版本
    Version,
    // 功能-文件路由（参数：路由配置）
    Route(RouteConfig),
}

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
        println!("it works!");
    }
}