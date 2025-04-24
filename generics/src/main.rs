fn main() {
    // let number_list = vec![34, 50, 25, 100, 65];
    // let result = largest(&number_list);
    // println!("The largest number is {}", result);

    // let mut p = Point {
    //     x: 1,
    //     y: 1,
    // };
    // println!("{}",p.x());
    // let mut p2 = Point {
    //     x: 1.1f32,
    //     y: 1.2f32,
    // };
    // let result2 = p2.distance_from_origin();
    // println!("{}",result2);

    let  x = mystery(3);
}

fn mystery<T>(x: T) -> T {
    x
}

struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//     for item in list {
//         if item > largest{
//             largest = item;
//         }
//     }
//     largest
// }