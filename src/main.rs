fn main() { 
    let array: [u8; 5] = [10, 20, 30, 40, 50];
    for elements in array { //* Optimized check
        println!("{elements}")
    }

}