use std::{path::PathBuf, str::FromStr};

use ip_check::ip_lookup::{Looker, IpLookup};
fn main(){
    let ip = "12.22.104.13";
    let file_path = PathBuf::from_str("locationv4.csv").expect("Path not correct");
    let looker = Looker::new(file_path);
    let result = looker.look_up(ip);
    match result {
        Some(ip_range) => {
            println!("Country: {}", ip_range.country);
            println!("Region: {}", ip_range.region);
            println!("City: {}", ip_range.city);
        },
        None => {
            println!("No match found");
        }
    }

    // let decimal = ip_to_decimal(ip);
    // println!("The decimal representation of {} is {}", ip, decimal);
    // println!("Hello, world!");
}
