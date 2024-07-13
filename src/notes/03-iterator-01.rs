fn main() {
    let values = vec![1, 2, 3];

    for v in values.into_iter() {
        println!("{}", v);
    }
    // 下面的代码将报错，因为 values 的所有权在上面 `for` 循环中已经被转移走
    // println!("{:?}", values);

    let values = vec![1, 2, 3];
    let _values_iter = values.iter();
    //  不会报错，因为 values_iter 只是借用了 values 中的元素
    println!("{:?}", values);

    let mut values = vec![1, 2, 3];
    // 对 values 中的元素进行可变借用
    let mut values_iter_mut = values.iter_mut();

    // 取出第一个元素，并修改为0
    if let Some(v) =  values_iter_mut.next() {
        *v = 0;
    }

    // 打印修改后的values
    println!("{:?}", values);
}