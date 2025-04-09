fn main() {
    // let y = {
    //   let x = 1;
    //     x + 1
    // };
    // println!("this value is : {y}");

    // let x = plus_one(5);
    // println!("x: {}", x);

    println!("x: {}", plus_one({
        let y = 1;
        y + 2
    }))
}

// fn plus_one(x: i32) -> i32 {
//     x + 1
// }

fn plus_one(x: i32) -> i32 { x + 1 }