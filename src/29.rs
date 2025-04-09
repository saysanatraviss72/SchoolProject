fn main() {
    let mut x = 10;
    while true {
        if x % 3 == 0 {
            println!("Fizz");
            break;
        } else if x % 5 == 0 {
            println!("Buzz");
            break;
        } else if x % 3 != 0 && x % 5 != 0 {
            println!("{}", x);
            break;
        }
        x += 1;
    }
}
