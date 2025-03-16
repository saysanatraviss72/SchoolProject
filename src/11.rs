use rand::prelude::*;

fn main() {
    let mut rng = thread_rng();
    let secret_code = rng.gen_range(100, 1000);
    println!("The secret code is: {}", secret_code);
}
