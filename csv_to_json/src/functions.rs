use crate::models::structs::HousePrice;
use csv::{Reader, Writer};

pub fn read_csv(path: String) -> Vec<HousePrice> {
    let mut rdr = Reader::from_path(path);
    vec![]
}