fn main() {
    let mut rng = rand::thread_rng();
    let roll1 = rng.gen_range(1, 7);
    let roll2 = rng.gen_range(1, 7);
    println!("{} + {}", roll1, roll2);
}
