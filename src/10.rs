use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("foo", "bar");
    println!("{}", map["foo"]);
}
