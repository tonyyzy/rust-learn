fn main() {
    // let mut s = String::new();
    // let data = "initial contents";
    // let s = data.to_string();
    // let s = String::from("initial contents");
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s1 = s1 + &s2;
    println!("{}", s1);
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);
}
