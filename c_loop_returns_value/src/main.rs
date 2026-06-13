fn main() {
    let mut counter = 0;

    let return_value = loop {
        println!("counter value: {:?}", counter);
        counter += 1;
        if counter == 10 {
            break 42;
        }
    };

    println!("return value: {:?}", return_value);
}
