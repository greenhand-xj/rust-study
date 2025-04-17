fn main() {
    let mut user1 = User {
        username: String::from("xujie"),
        email: String::from("12312@qq.com"),
        active: true,
        sign_in_count: 0,
    };
    let user2 = User {
        username: String::from("lihao"),
        ..user1
    };
    // user1.email.push_str(&user2.email);
    println!("{:?}", user2);
    // let mut p = Point { x: 0, y: 0 };
    //
    // let x = &mut p.x;
    //
    // p.y = 1;
    // println!("y is {}", p.y);
    //
    // *x+=1;

    // println!("{p:#?}");

    // let rect1 = Rectangle {
    //     width: 10,
    //     height: 20
    // };
    // let mut rect2 = Rectangle {
    //     width: 4,
    //     height: 5
    // };
    // println!("rect1 area: {}", rect1.area());
    // println!("rect2 area: {}", rect2.area());
    // println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));
    //
    // rect2.width = 30;
    //
    // let mut a = Point {
    //     x: 1,y:2
    // };
    // a.x += 1;
    // let b = Point {
    //     y:1,
    //     ..a
    // };
    // a.x += 1;
    // println!("{:?}", b.x);


    let mut p = Point {
        x: 1,y:2
    };
    let x = &mut p.x;
    let y = &mut p.y;

    *x += 1;
    *y += 1;
    println!("{}", p.x);
    println!("{}", p.y);
}

// struct Rectangle {
//     width: u32,
//     height: u32,
// }
// impl Rectangle {
//    fn area(&self) -> u32 {
//        self.width * self.height
//    }
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
//    // fn new(width: u32, height: u32) -> Self {
//    //     Self::new(width, height)
//    // }
// }

#[derive(Debug)]
struct Point {x: i32,y:i32}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
