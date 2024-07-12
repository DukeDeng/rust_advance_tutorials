fn main(){
    let init_x: i32 = 2; // 这个用来判断返回的是哪个函数
    let f = factory(init_x);
    let answer = f(3);
    assert_eq!(answer, 12);
}

fn  factory(x: i32) -> Box<dyn Fn(i32) -> i32> {
    let m = 10;
    if x > 1 { // init_x
        Box::new(move |y| y + m)
    } else {
        Box::new(move |y| y - m)
    }
}