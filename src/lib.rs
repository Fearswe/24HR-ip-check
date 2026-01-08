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

    #[derive(Debug, Default)]
    pub struct LookerBuilder {
        file_path: Option<PathBuf>,
        allowed_countries: Option<Vec<String>>,
    }

    pub trait IpLookup {
        fn look_up(&self, ip: &str) -> Option<IpRange>;
        fn look_up_ipv4(&self, ip: &Ipv4Addr) -> Option<IpRange>;
    }

    impl Looker {

        pub fn new(file_path: PathBuf) -> Self {

            let ip_ranges = match read_ip_ranges(file_path.to_str().expect("IP CSV file not found"), None) {
                Ok(ranges) => ranges,
                Err(e) => {
                    log::error!("Error reading IP ranges: {}", e);
                    Vec::new()
                }
            };
            Looker {
                file_path,
                ip_ranges,
            }

        }

        pub fn builder() -> LookerBuilder {
            LookerBuilder::new()
        }

    }

    impl LookerBuilder {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn file_path(mut self, path: PathBuf) -> Self {
            self.file_path = Some(path);
            self
        }

        pub fn allowed_countries(mut self, countries: Vec<String>) -> Self {
            if countries.is_empty() {
                log::warn!("Allowed countries is empty, filter will be ignored!");
                self.allowed_countries = None;
                return self;
            }

            self.allowed_countries = Some(countries);
            self
        }

        pub fn build(self) -> Result<Looker, Box<dyn Error>> {
            let ip_ranges = match read_ip_ranges(self.file_path.as_ref().expect("IP CSV file not found").to_str().expect("Invalid file path"), self.allowed_countries.as_ref()) {
                Ok(ranges) => ranges,
                Err(e) => {
                    log::error!("Error reading IP ranges: {}", e);
                    Vec::new()
                }
            };

            Ok(Looker {
                file_path: self.file_path.unwrap(),
                ip_ranges,
            })
        }
    }

    fn read_ip_ranges(file_path: &str, allowed_countries: Option<&Vec<String>>) -> Result<Vec<IpRange>, Box<dyn Error>> {
        let mut rdr = Reader::from_path(file_path)?;
        let mut ip_ranges = Vec::new();

        let allowed_countries = match allowed_countries {
            Some(filter) => {
                if filter.is_empty() {
                    log::warn!("Country filter is empty, filter will be ignored!");
                    None
                } else {
                    Some(filter)
                }
            },
            None => None,
        };

        for result in rdr.records() {
            let record = result?;
            let start: u32 = record[0].parse()?;
            let end: u32 = record[1].parse()?;
            let country = record[2].to_string();
            let region = record[4].to_string();
            let city = record[5].to_string();

            if let Some(ref filter) = allowed_countries {
                if !filter.contains(&country) {
                    continue;
                }
            }
            
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
         let ip_ranges_to_use = match read_ip_ranges(file_path, None) {
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

    pub fn look_up_filtered(ip: &str, file_path: &str, allowed_countries: &Vec<String>) -> Option<IpRange> {
        let ip_decimal_to_use = match ip_string_to_decimal(ip) {
            Err(e) => {
                log::error!("Error: {}", e);
                return None;
            },
            Ok(ip_decimal) => {
                ip_decimal
            }
        };
         let ip_ranges_to_use = match read_ip_ranges(file_path, Some(allowed_countries)) {
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

pub use crate::ip_lookup::{look_up, look_up_filtered, Looker, LookerBuilder, IpLookup, };
