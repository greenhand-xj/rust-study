use std::fmt::Display;

fn displayable<T: Display>(t: T) -> impl Display {
    t
}
fn main(){
    let s = String::from("hello world");
    let mut s2 = displayable(s);
    println!("{}", s2);
}