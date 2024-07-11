fn fn_once<F>(func: F)
 where 
    F: FnOnce(usize) -> bool + Copy,
{
    println!("{}", func(3));
    println!("{}", func(4));
}

fn main() {
    let x = vec![1,2,3];
    fn_once(|z|{z == x.len()});
}

// simple source code
// pub trait Fn<Args> : FnMut<Args> {
//     extern "rust-call" fn call(&self, Args) -> Self::Output;
// }

// pub trait FnMut<Args> : FnOnce<Args> {
//     extern "rust-call" fn call_mut(&mut self, Args) -> Self::Output;
// }

// pub trait FnOnce<Args> {
//     type Output;

//     extern "rust-call" fn call_once(self, Args) -> Self::Output;
// }
