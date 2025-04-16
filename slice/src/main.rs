fn main() {
    // let a = b'a';
    // println!("{}",a);
    // let mut s = String::from("hello world");
    // let word = first_word(&s);
    // // s.clear();
    // println!("{}",word);

    // let a = [1,2,3,4,5];
    //
    // let slice = &a[1..3];
    //
    // assert_eq!(slice, &[2,3]);

    // let mut s = String::from("hello");
    // for &item in s.as_bytes().iter() {
    //     if item == b'l' {
    //         s.push_str(" world");
    //     }
    // }
    // println!("{}", s);

    // println!("{:?}", num);

        let s = String::from("hello world1111");
        let s2: &String = &s;
        let s3: &str = &s[..];
        println!("s2 size: {} bytes", std::mem::size_of_val(s2));
        println!("s3 size: {} bytes", std::mem::size_of_val(s3));
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[..i];
        }
    }
    return &s[..];
}
