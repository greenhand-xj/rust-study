fn main() {
    // let condition = true;
    //
    // let x = if condition { 2 } else { 1 };
    //
    // let y;
    // if condition {
    //     y = 2;
    // } else {
    //     y = 1;
    // }
    // println!("x: {}", x);
    // println!("y: {}", y);

    // let y = 1;
    // let x = if y {1} else {0};
    // println!("x: {}", x);

    // let mut x = 0;
    // 'a: loop {
    //     x += 1;
    //     'b: loop {
    //         if x > 10 {
    //             continue 'a;
    //         }else {
    //             break 'b;
    //         }
    //     }
    //     break;
    // }
    // println!("x: {}", x);

    let a = [5;10];
    let mut sum = 0;
    for x in a {
        sum += x;
    }
    println!("sum: {}", sum);
}
