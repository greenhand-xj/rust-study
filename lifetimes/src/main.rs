fn main() {
    let s1 = String::from("hello world");
    let s2 = "hello world1";
    let lone = longest(s1.as_str(), s2);
    println!("{}",lone);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    }else {
        y
    }
}
