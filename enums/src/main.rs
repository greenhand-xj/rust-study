use std::ptr::null;

fn main() {
    // println!("Hello, world!");


    // let five = Some(5);

    // let val = Some(String::from("Hello, world!"));
    //
    // match &val {
    //     None => println!("None"),
    //     Some(s) => println!("{}", s),
    // }
    //
    // println!("{:?}",val);

    // let x = null;

    // enum Location {
    //     Point(i32),
    //     Range(i32, i32),
    // }
    // let l: Location = Location::Range(0, 5);
    // let n = match l {
    //     Location::Point(_) => -1,
    //     Location::Range(_, n) => n,
    //     Location::Range(0,_) => 0,
    //     _ => -2
    // };
    // println!("{}", n);

    #[derive(Debug)]
    enum Either {
        Left(usize),
        Rigth(String),
    }
    let x = Either::Rigth(String::from("Hello, world!"));

    let value = match &x {
        Either::Left(n) => n,
        Either::Rigth(s) => &s.len(),
    };

    println!("{:?} {value}", x);
}
