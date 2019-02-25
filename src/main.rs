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
    name: String,
    description: String,
    country_code: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Prefixes {
    ipv4_prefixes: Vec<Prefix>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    status: String,
    status_message: String,
    data: Prefixes,
}

fn main() -> io::Result<()> {
    for file in ["as4538_prefixes", "as23910_prefixes"].iter() {
        let mut f = File::open(file)?;
        let mut buffer = String::new();
        f.read_to_string(&mut buffer)?;
        let data: Data = serde_json::from_str(&buffer).unwrap();
        let mut vec: Vec<IPAddress> = Vec::new();
        for prefix in data.data.ipv4_prefixes.iter() {
            if prefix.name.contains("TSINGHUA")
                || prefix.name.contains("TUZJ")
                || prefix.name.contains("TUNET")
            {
                //println!("{}", prefix.prefix);
                vec.push(IPAddress::parse(prefix.prefix.clone()).unwrap());
            }
        }

        let aggregated = IPAddress::aggregate(&vec);

        for prefix in aggregated.iter() {
            println!("{}", prefix.to_string());
        }
    }

    Ok(())
}
