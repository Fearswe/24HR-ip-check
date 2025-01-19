pub mod ip_lookup {

    use std::net::Ipv4Addr;
    use std::path::PathBuf;
    use std::str::FromStr;
    use std::cmp::Ordering;
    use std::error::Error;
    use csv::Reader;


    #[derive(Debug, Clone)]
    pub struct IpRange {
        start: u32,
        end: u32,
        pub country: String,
        pub region: String,
        pub city: String,
    }

    #[derive(Debug)]
    pub struct Looker {
        pub file_path: PathBuf,
        pub ip_ranges: Vec<IpRange>,
    }

    pub trait IpLookup {
        fn look_up(&self, ip: &str) -> Option<IpRange>;
        fn look_up_ipv4(&self, ip: &Ipv4Addr) -> Option<IpRange>;
    }

    impl Looker {

        pub fn new(file_path: PathBuf) -> Self {

            let mut rdr = Reader::from_path(&file_path).expect("IP CSV file not found");
            let mut ip_ranges = Vec::new();

            for result in rdr.records() {
                let record = result.unwrap();
                let start: u32 = record[0].parse().unwrap();
                let end: u32 = record[1].parse().unwrap();
                let country = record[2].to_string();
                let region = record[4].to_string();
                let city = record[5].to_string();

                ip_ranges.push(IpRange { start, end, country, region, city });
            }

            Looker {
                file_path,
                ip_ranges,
            }

        }

    }

    fn read_ip_ranges(file_path: &str) -> Result<Vec<IpRange>, Box<dyn Error>> {
        let mut rdr = Reader::from_path(file_path)?;
        let mut ip_ranges = Vec::new();
        
        for result in rdr.records() {
            let record = result?;
            let start: u32 = record[0].parse()?;
            let end: u32 = record[1].parse()?;
            let country = record[2].to_string();
            let region = record[4].to_string();
            let city = record[5].to_string();
            
            ip_ranges.push(IpRange { start, end, country, region, city });
        }

        Ok(ip_ranges)
    }

    fn find_ip_range(ip: u32, ranges: &[IpRange]) -> Option<IpRange> {
        ranges.binary_search_by(|range| {
            if ip < range.start {
                Ordering::Greater // Search the left side
            } else if ip > range.end {
                Ordering::Less // Search the right side
            } else {
                Ordering::Equal // IP is within this range
            }
        }).ok().map(|index| ranges[index].clone())
    }

    fn ip_string_to_decimal(ip: &str) -> Result<u32, String> {
        let ip = Ipv4Addr::from_str(ip);
        if ip.is_err() {
            return Err("Invalid IP address".into());
        }
        let ip = ip.unwrap();
        ip_to_decimal(&ip)
    }

    fn ip_to_decimal(ip: &Ipv4Addr) -> Result<u32,String> {
        let octets = ip.octets();
        let decimal = (octets[0] as u32) << 24 
            | (octets[1] as u32) << 16 
            | (octets[2] as u32) << 8 
            | octets[3] as u32;
        Ok(decimal)
    }


    pub fn look_up(ip: &str, file_path: &str) -> Option<IpRange> {
        let ip_decimal_to_use = match ip_string_to_decimal(ip) {
            Err(e) => {
                log::error!("Error: {}", e);
                return None;
            },
            Ok(ip_decimal) => {
                ip_decimal
            }
        };
         let ip_ranges_to_use = match read_ip_ranges(file_path) {
            Err(e) => {
                log::error!("Error: {}", e);
                return None;
            },
            Ok(ip_ranges) => {
                ip_ranges
            }
        };
        
        match find_ip_range(ip_decimal_to_use, &ip_ranges_to_use[..]) {
            Some(range) => {
                log::trace!("IP is in range: {:?}", range);
                Some(range)
            },
            None => {
                log::trace!("IP not found in any range");
                None
            }
        }
    }

    impl IpLookup for Looker {

        fn look_up(&self, ip: &str) -> Option<IpRange> {
            let ip = Ipv4Addr::from_str(ip);
            match ip {
                Err(e) => {
                    log::error!("Error: {}", e);
                    None
                },
                Ok(ip) => {
                    self.look_up_ipv4(&ip)
                }
            }
 
       }

        fn look_up_ipv4(&self, ip: &Ipv4Addr) -> Option<IpRange> {

            let ip_decimal_to_use = match ip_to_decimal(ip) {
                Err(e) => {
                    log::error!("Error: {}", e);
                    return None;
                },
                Ok(ip_decimal) => {
                    ip_decimal
                }
            };
            let ip_ranges_to_use = &self.ip_ranges;

            match find_ip_range(ip_decimal_to_use, &ip_ranges_to_use[..]) {
                Some(range) => {
                    log::trace!("IP is in range: {:?}", range);
                    Some(range)
                },
                None => {
                    log::trace!("IP not found in any range");
                    None
                }
            }
        }

    }

}

pub use crate::ip_lookup::{look_up, Looker, IpLookup};
