// implement a prime sieve

fn is_prime(num: i32) -> bool {
    for i in 2..=((num as f64).sqrt() as i32) {
        if num % i == 0 {
            return false
        }
    }
    return true
}
fn main() {
    // println!("{}", (45i32 as f64).sqrt());
    for i in 2..=1000000 {
        // match is_prime(i) {
        //     true => println!("{}", i),
        //     _ => ()
        // }
        is_prime(i);
        
    }
}