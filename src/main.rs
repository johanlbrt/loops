fn main() { 
    let s1 = String::from("hello");
    let s2 = s1.clone(); // proper cloning without stack clearing
    println!("{s1}{s2}");
}