use std::thread;
use std::time::Duration;
fn workout(intensity: u32, random_number: u32){
    let action = || {
        println!("muuuu...");
        thread::sleep(Duration::from_secs(2));
        intensity
    };

    if intensity < 25 {
        println!("今天活力满满，先做 {} 个俯卧撑!", action());
        println!("然后再做 {} 次深蹲!", action());
    } else if random_number == 3 {
        println!("昨天练过了休息一下！");
    } else {
        println!("今天的活力不够，先做 {} 次深蹲!", action());
    }
}

fn main() {
    // 强度
    let intensity = 10;

    // 随机数来决定某个选择
    let random_number = 7;

    // 猛起
    workout(intensity, random_number);
}