  pub fn random_code() -> String {
    let mut rng = rand::thread_rng();
    let random_index: usize = rng.gen_range(0..10);
  
    match random_index {
      0 => "fn main() {\n\tprintln!(\"Hello, world!\");\n}",
      1 => "let x = 5;\nlet y = 3;\nlet z = x + y;",
      2 => "for i in 0..5 {\n\tprintln!(i);\n}",
      3 => "if 7 > 3 { println!(\"7 is greater than 3\"); } else { println!(\"7 is less than or equal to 3\"); }",
      4 => "let mut vec = Vec::new();\nvec.push(1);\nvec.push(2);\nvec.push(3);\nprintln!(vec[0]);",
      5 => "enum Color {\n\tRed,\n\tGreen,\n\tBlue,\n}",
      6 => "struct Point<T> { x: T, y: T }",
      7 => "fn add_one(x: i32) -> i32 { x + 1 }",
      8 => "use std::collections::HashMap; \nlet mut my_map = HashMap::new(); \nmy_map.insert(1, \"a\"); \nmy_map.insert(2, \"b\");",
      9 => "macro_rules! debug { ($expr:expr) => { println!(\"{:?}\", $expr); } } ",
      _ => "// Generated a random rust code snippet for your project."
    }
  }