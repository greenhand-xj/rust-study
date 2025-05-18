use std::sync::Arc;
use std::thread;

fn main() {
    let s = String::from("hello world");
    let a = Arc::new(&s);
    let a2 = Arc::clone(&a);
    let t = thread::spawn(move || a2.len());
    let len = t.join().unwrap();
    println!("{} {}",a,len);
}
