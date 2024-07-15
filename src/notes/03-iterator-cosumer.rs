fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    let total :i32 =  v1_iter.sum();
    println!("The sum is: {}", total);

    assert_eq!(total, 6);

    // v1_iter 是借用了 v1，因此 v1 可以照常使用
    println!("The vector is: {:?}", v1);

    // 以下代码会报错，因为 `sum` 拿到了迭代器 `v1_iter` 的所有权
    // println!("{:?}",v1_iter);
    let v2: Vec<i32> = vec![1, 2, 3];

    let v3: Vec<_> = v2.iter().map(|x| x + 1).collect();

    assert_eq!(v3, vec![2, 3, 4]);

}