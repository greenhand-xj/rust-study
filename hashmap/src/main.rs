use std::collections::hash_map::Entry;
use std::collections::HashMap;

fn main() {
    // println!("Hello, world!");
    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);
    //
    // let vec = vec![("key","value1"),("key2", "value2")];
    // let mut map: HashMap<_,_> = vec.into_iter().collect();
    //
    // for (key,valus) in &scores {
    //     println!("{}: {}", key, valus);
    // }
    //
    // println!("{:?}", scores);

    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // scores.entry(String::from("Yellow")).or_insert(50);
    // scores.entry(String::from("Blue")).or_insert(50);
    //
    // println!("{:?}", scores);

    // let text = "hello world hello word";
    // let mut map = HashMap::new();
    // for word in text.split_whitespace() {
    //     let count = map.entry(word).or_insert(0);
    //     *count += 1;
    // }
    // // let c = &map['word']
    // println!("{:#?}", map);

    let mut h = HashMap::new();
    h.insert("k1", 0);
    let v1 = &h["h1"];
    h.insert("k2", 1);
    let v2 = &h["h2"];
    println!("{v1} {v2}");
}
