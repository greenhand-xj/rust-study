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


    // let mut v = vec![1,2,3];
    //
    // let mut num = &v[2];
    // let z = 4;
    // num = &z;
    // println!("num: {}", *num);
    // println!("vec: {}", v[2]);
    //
    // v.push(4);

    // let mut v = vec![1,2,3]; // RWO
    // let num = &v[2]; //RO v-W -O
    // // v.push(4);
    // println!("{:?}", num);
    // println!("{:p}", &num);
    // println!("{:p}", &v[2]);

    // let mut v = vec![1,2,3];
    // let num = &mut v[2];
    // // v[2] = 4;
    // // *num = 5;
    //
    // let num2 = &*num;
    // println!("{} {}", *num2,*num);
    // let mut n = 1;
    // incr(&mut n);
    // println!("{}", n);
    // let mut s = String::from("hello");
    // let s2 = &s;
    // // let s3 = &mut s;
    // // s3.push_str(", world");
    // s.push_str(", world");
    // println!("{}", s2);

    let mut v = vec![1,2,3];
    let n = &v[0];
    give_and_take(&mut v,4);
    println!("{}",n);
}

fn give_and_take(v: &mut Vec<i32>,n: i32) -> i32 {
    v.push(n);
    v.remove(0)
}

// fn incr(n: &mut i32){
//     *n += 1;
// }

// fn greet(g1: &String, g2: &String)  {
//     println!("{} {}", g1,g2);
// }
