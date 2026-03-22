fn greet(name: &str) -> String {
    format!("hello {}", name)
}

fn main() {
    let msg = greet("world");
    let msg2 = greet("world");
    println!("{}", msg);
    println!("{}", msg2);
}
