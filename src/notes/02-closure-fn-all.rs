fn main (){
    let s = String::new();

    let update_string = || println!("{}", s);

    exec(update_string);
    exec1(update_string);
    exec2(update_string);
}

fn exec<F: FnOnce()>(f:F){
    f()
}

fn exec1<F: FnMut()>(mut f:F){
    f()
}

fn exec2<F: Fn()>(f:F){
    f()
}