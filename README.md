This crate exposes a struct called Looker that you need to instantiate like this:

```rust
use ip_check::{Looker, IpLookup};

let file_path = PathBuf::from_str("locationv4.csv").expect("Path not correct");
let looker = Looker::new_(file_path);
```

You can then use the looker variable to call the look_up method like this to get a IP result with Country, Region and City:

```rust
    let ip = Ipv4Addr::new(12, 22, 104, 13);
    let result = looker.look_up_ipv4(ip);
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
```
Or if you only have a string as an IP:

```rust
    let ip = "12.22.104.13";
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
```
You can download the ip .csv file from here:
https://cable.ayra.ch/ip/data/locationv4.gz

And then unzip it with:
```
gzip -d locationv4.gz
```

The file needs to have the .csv extension. If it doesn't have it after you unzip it, add it.

## History

- 0.1.5 - Breaking change: Needs a PathBuf instead of a string to load the csv. New function called look_up_ipv4 that takes an Ipv4Addr instead of a string.
          


