fn main(){
    let x = 1;
    let sum = |y| x + y;
    assert_eq!(sum(2), 3);
}