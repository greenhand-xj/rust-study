use std::fmt::Display;

pub struct Pair<T> {
    x: T,
    y: T,
}

impl <T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The result is {}", self.x);
        }else { 
            println!("The result is {}", self.y);
        }
    }
}