fn main() {
    let mut counter = 0;
    let ten = loop {
        counter = counter + 1;
        if counter == 10 {
            break counter;
        }
    };
    println!("{}", ten);
}