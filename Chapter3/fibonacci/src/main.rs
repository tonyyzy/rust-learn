fn fibonacci(n: i32) -> i32 {
    if n == 2 {
        return 1
    } else if n == 1 {
        return 1
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2)
    }
}


fn main() {
    println!("fib 40 is {}", fibonacci(40));
}
