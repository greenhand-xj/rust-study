fn main() {
    // let mut v = vec![100,200,300];
    // for n_ref in &mut v {
    //     *n_ref += 200;
    // }
    // println!("{:?}", v);

    // let v = vec![String::from("wode")];
    // let mut s = v[0];

    // let mut v = vec![1,2,3];
    // for i in &mut v {
    //     v.push(*i);
    // }

    let mut v = vec![1,2,3];
    let mut v2: Vec<&mut i32> = Vec::new();

    for i in &mut v {
        v2.push(i);
    }
    *v2[0] = 5;

    let a = *v2[0];
    let b = v[0];
    println!("{a} {b}");
}
