

use rand::SeedableRng;

use crate::{record::DNSRecordResult, send_query, TYPE_A};

fn resolve2(domain_name: &str, record_type: u16) -> eyre::Result<DNSRecordResult> {
    let rng = &mut rand::rngs::SmallRng::from_entropy();
    let mut nameserver = "198.41.0.4".parse().unwrap();
    let mut domain_names = vec![domain_name.to_string()];

    let ip = loop {
        println!("Querying {nameserver} for {}", domain_names[0]);
        let response = send_query(rng, nameserver, domain_names.last().unwrap(), record_type)?;

        if let Some(ip @ DNSRecordResult::Address(a)) = response.get_answer() {
            if domain_names.len() > 1 {
                domain_names.pop();
                nameserver = *a;
            } else {
                break ip.clone();
            }
        } else if let Some(DNSRecordResult::Address(ns_ip)) = response.get_nameserver_ip() {
            nameserver = *ns_ip;
        } else if let Some(DNSRecordResult::NameServer(ns)) = response.get_nameserver() {
            domain_names.push(ns.to_string());
        } else {
            return Err(eyre::eyre!("No answer or nameserver found"));
        }
    };
    Ok(ip)
}

fn resolve(domain_name: &str, record_type: u16) -> eyre::Result<DNSRecordResult> {
    let rng = &mut rand::rngs::SmallRng::from_entropy();
    let mut nameserver = "198.41.0.4".parse().unwrap();
    let ip = loop {
        println!("Querying {nameserver} for {domain_name}");
        let response = send_query(rng, nameserver, domain_name, record_type)?;
        if let Some(ip) = response.get_answer() {
            break ip.clone();
        } else if let Some(DNSRecordResult::Address(ns_ip)) = response.get_nameserver_ip() {
            nameserver = *ns_ip;
        } else if let Some(DNSRecordResult::NameServer(ns)) = response.get_nameserver() {
            let r = resolve(ns, TYPE_A)?;
            match r {
                DNSRecordResult::Address(ns_ip) => nameserver = ns_ip,
                _ => return Err(eyre::eyre!("No IP found for nameserver")),
            }
        } else {
            return Err(eyre::eyre!("No answer or nameserver found"));
        }
    };
    Ok(ip)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve() {
        let res = resolve2("twitter.com", TYPE_A).unwrap();
        println!("{:?}", res);
    }
}
