#[macro_use]
extern crate serde_derive;

use ipaddress::IPAddress;
use std::fs::File;
use std::io;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
struct Prefix {
    prefix: String,
    ip: String,
    cidr: u32,
    roa_status: String,
    name: Option<String>,
    description: Option<String>,
    country_code: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Prefixes {
    ipv4_prefixes: Vec<Prefix>,
    ipv6_prefixes: Vec<Prefix>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    status: String,
    status_message: String,
    data: Prefixes,
}

fn main() -> io::Result<()> {
    let mut vec: Vec<IPAddress> = Vec::new();
    for file in [
        "as4538_prefixes",
        "as23910_prefixes",
        "as24348_prefixes",
        "as45576_prefixes",
    ]
    .iter()
    {
        let mut f = File::open(file)?;
        let mut buffer = String::new();
        f.read_to_string(&mut buffer)?;
        let data: Data = serde_json::from_str(&buffer).unwrap();
        for prefix in data.data.ipv4_prefixes.iter() {
            if let Some(name) = &prefix.name {
                if name.contains("TSINGHUA") || name.contains("TUZJ") || name.contains("TUNET") {
                    vec.push(IPAddress::parse(prefix.prefix.clone()).unwrap());
                }
            }
        }
        for prefix in data.data.ipv6_prefixes.iter() {
            if let Some(name) = &prefix.name {
                if name.contains("TSINGHUA") || name.contains("TUZJ") || name.contains("TUNET") {
                    vec.push(IPAddress::parse(prefix.prefix.clone()).unwrap());
                }
            }
        }
    }
    let aggregated = IPAddress::aggregate(&vec);

    for prefix in aggregated.iter() {
        println!("{}", prefix.to_string());
    }

    Ok(())
}
