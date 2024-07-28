use std::convert::TryInto;
use std::sync::Arc;

fn main(){
    let a = 3.1 as i8;
    let b = 100_i8 as i32;
    let c = 'a' as u8;// 将字符'a'转换为整数，97

    println!("a: {}, b: {}, c: {}", a, b, c);

    let mut values: [i32; 2] = [1, 2];
    let p1: *mut i32 = values.as_mut_ptr();
    let first_address = p1 as usize; // 将p1内存地址转换为一个整数
    let second_address = first_address + 4; // 4 == std::mem::size_of::<i32>()，i32类型占用4个字节，因此将内存地址 + 4
    let p2 = second_address as *mut i32; // 访问该地址指向的下一个整数p2
    unsafe {
        *p2 += 1;
    }
    assert_eq!(values[1], 3);

    let a: u8 = 10;
    let b: u16 = 100;

    // let b_: u8 = b.try_into().unwrap();
    let b_: u8 = match b.try_into() {
        Ok(b1) => b1,
        Err(e) => {
            println!("{:?}", e.to_string());
            0
        }
    };

    if a < b_ {
        println!("Ten is less than one hundred.");
    }

    let pointer = foo as *const ();
    let function = unsafe {
        // 将裸指针转换为函数指针
        std::mem::transmute::<*const (), fn() -> i32>(pointer)
    };
    assert_eq!(function(), 0);
}

#[derive(Clone)]
struct Container<T>(Arc<T>);

fn clone_containers<T>(foo: &Container<i32>, bar: Container<T>) where T: Clone {
    let _foo_clone = foo.clone();
    let _bar_clone = bar.clone();
}

fn foo() -> i32 {
    0
}

struct Foo {
    x: u32,
    y: u16,
}

struct Bar {
    a: u32,
    b: u16,
}

fn reinterpret(foo: Foo) -> Bar {
    let Foo { x, y } = foo;
    Bar { a: x, b: y }
}
