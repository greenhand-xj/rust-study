use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("秘密数字是：{}", secret_number);

    loop {
        println!("请输入一个数字");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("失败");
        println!("你猜的是：{}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(v) => v,
            Err(_) => continue,
        };
        // 真TM优雅
        match guess.cmp(&secret_number) {
            Ordering::Less => print!("小了"),
            Ordering::Greater => print!("大了"),
            Ordering::Equal => {
                print!("你猜中了");
                break;
            }
        };
    }
}
