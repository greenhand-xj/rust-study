use List::{Cons,Nil};

fn main() {
    // let list = Cons(1, Box::from(Cons(2, Box::from(Nil))));
    let mut n = 1;
    let b = Box::new(&mut n);
    **b += 1;
    println!("{}", n);
}

pub enum List {
    Cons(i32, Box<List>),
    Nil
}
