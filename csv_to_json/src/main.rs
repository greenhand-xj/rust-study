use csv_to_json::functions::*;
use csv_to_json::models::enums::YesNo;
use csv_to_json::models::structs::HousePrice;

fn main() {
    // println!("Hello, world!");
    let y = YesNo::Yes;
    let house_price = HousePrice {
     bed_rooms: 2,
     price: 10000,
        area: String::from("wode"),
        main_road: YesNo::No,
    };

    read_csv(String::from(r"C:\Users\user\house\house.csv"));
}