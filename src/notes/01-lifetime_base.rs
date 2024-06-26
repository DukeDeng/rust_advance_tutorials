#[warn(unused_mut)]
fn main(){
    let string1 = String::from("hello");
    let result;
    {
        let string2 = String::from("world");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is: {}", result); 
    }
}

// 修改后，y 完全没有被使用，因此 y 的生命周期与 x 和返回值的生命周期没有任何关系，
// 意味着我们也不必再为 y 标注生命周期，只需要标注 x 参数和返回值即可。
fn longest<'a>(x: &'a str, _y: &str) -> &'a str {
    // if x.len() > y.len() {
    //     x
    // } else {
    //     y
    // }
    x
}

fn longest01<'a>(_x: &str, _y: &str) -> String {
    // 错误示例
    // let result = String::from("really long string");
    // result.as_str() 
    // 函数的返回值如果是一个引用类型，那么它的生命周期只会来源于：
    // - 函数参数的生命周期
    // - 函数体中某个新建引用的生命周期
    // result 在函数结束后就被释放，但是在函数结束后，对 result 的引用依然在继续。在这种情况下，没有办法指定合适的生命周期来让编译通过，因此我们也就在 Rust 中避免了悬垂引用。


    // 那遇到这种情况该怎么办？最好的办法就是返回内部字符串的所有权，然后把字符串的所有权转移给调用者：
    // 正确
    String::from("really long string")
}