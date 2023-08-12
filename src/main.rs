fn main() {
    let array: [u8; 5] = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < array.len() {
        println!("Array index {index} = {}", array[index]);
        index += 1;
        
    }

}