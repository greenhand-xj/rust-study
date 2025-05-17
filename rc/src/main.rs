enum List {
    Cons(i32, Rc<List>),
    Nil
}

use std::rc::Rc;
use List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil)))))));
    let b = Cons(4, Rc::clone(&a));
    let c = Cons(5, Rc::clone(&a));
}
