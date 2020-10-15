struct Config {
    path: Vec<&'static str>,
    func: Func,
}

enum Func {
    Find(&'static str), Compare(&'static str, &'static str),
}

impl Config {
    fn from_args<'a>(args: &'a [&str]) -> Result<Self, &'a str> {
        Err("something wrong parsing arguments")
    }

}