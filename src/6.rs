fn main() {
    let mut rng = rand::thread_rng();
    let num: i32 = rng.gen_range(1..100);
    println!("{}", num);
}
