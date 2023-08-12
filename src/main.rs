fn main() { //? Number devision by 2023?
    let mut number:u128 = 0;
    let mut counter:u8 = 0;
    
    // * There is no efficient overshoot
    loop {
        number+=1;
        if number % 2023 == 0 {
            println!("{number}");
            counter+=1;
            if counter == 10 {break};
        }

    }
}