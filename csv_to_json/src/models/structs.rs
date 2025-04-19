use crate::models::enums::YesNo;

pub struct HousePrice {
    pub price: i32,
    pub area: String,
    pub bed_rooms: u32,
    pub main_road: YesNo
}