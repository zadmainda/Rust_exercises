pub fn apply(f: fn(i32) -> i32, a: i32) {
    println!("result {}", f(a));
}
