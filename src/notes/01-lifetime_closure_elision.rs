fn main() {
    let closure_elision = func(|x: &i32| -> &i32 { x });
    assert_eq!(closure_elision(&42), &42);
}

fn func<T, F: Fn(&T) -> &T>(f: F) -> F {
    f
}