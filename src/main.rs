fn main() { 
    for tick in (1..6).rev() {
        println!("{tick}");
        if tick == 1 {
            println!("bye.")
        }
    }
    println!("BOOM");
}