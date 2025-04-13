fn main() {
    // let m1 = String::from("hellow");
    // let m2 = String::from("world");
    // greet(&m1,&m2);
    //
    // let s = format!("{} {}", m1,m2);

    // let x = Box::new(5);
    // let y = Box::new(x);

    // println!("y: {}", *y);

    // let x = Box::new(5);
    // let y = x;

    // println!("{}", **&&y);

    // println!("{}", *x);

    // let r1 = &y;
    // let r2 = &y;
    // println!("r1: {}", **r1);
    // println!("r2: {}", r2);


    let mut v = vec![1,2,3];

    let mut num = &v[2];
    let z = 4;
    num = &z;
    println!("num: {}", *num);
    println!("vec: {}", v[2]);

    v.push(4);
}

// fn greet(g1: &String, g2: &String)  {
//     println!("{} {}", g1,g2);
// }
