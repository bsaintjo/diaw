mod encoding;
mod header;
pub mod packet;
mod query;
mod question;
mod record;
pub mod resolve;

pub const TYPE_A: u16 = 1;
pub const CLASS_IN: u16 = 1;

#[cfg(test)]
mod test {
    use std::{net::IpAddr, str::FromStr};
    use rand::SeedableRng;
    use crate::query::send_query;
    use super::*;

    #[test]
    fn test_main() {
        let rng = &mut rand::rngs::SmallRng::from_entropy();
        let ip_addr = IpAddr::from_str("198.41.0.4").unwrap();
        let domain_name = "www.google.com";
        let record_type = TYPE_A;
        let response = send_query(rng, ip_addr, domain_name, record_type).unwrap();
        println!("Authorities {:#?}", response.authorities());
        println!("Additionals {:#?}", response.additionals());

        let response = send_query(
            rng,
            "192.12.94.30".parse().unwrap(),
            domain_name,
            record_type,
        )
        .unwrap();
        println!("Authorities {:#?}", response.authorities());
        println!("Additionals {:#?}", response.additionals());
        let response = send_query(
            rng,
            "216.239.32.10".parse().unwrap(),
            domain_name,
            record_type,
        )
        .unwrap();
        println!("Answer {:#?}", response.answers());
    }
}
