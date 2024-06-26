use std::boxed::Box;

#[derive(Debug)]
struct Config {
    a: String,
    b: String,
}
fn init() -> Option<&'static mut Config> {
    
    //  创建config 实例 放在box中
    let config = Box::new(Config {
        a: "A".to_string(),
        b: "B".to_string(),
    });
    
    // 通过泄露 box, 获取其内容的 'static 生命周期的可变引用
    let leaf_config: &'static mut Config = Box::leak(config);
    
    // 修改 config 的内容
    Some(leaf_config)
}

fn main() {
    unsafe {
        let config = init();
        println!("{:?}", config);
    }
}