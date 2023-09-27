use std::net::IpAddr;

use rand::SeedableRng;

use crate::{r#async::query::send_query_async, record::DNSRecordResult};

pub async fn resolve_async(domain_name: &str, record_type: u16) -> eyre::Result<DNSRecordResult> {
    tracing::debug!("Resolving {} for type {}", domain_name, record_type);
    let rng = &mut rand::rngs::SmallRng::from_entropy();
    let mut domain_names = vec![domain_name.to_string()];
    let mut nameserver: IpAddr = "198.41.0.4".parse().unwrap();

    let ip = loop {
        let response =
            send_query_async(rng, nameserver, domain_names.last().unwrap(), record_type).await?;
        tracing::debug!("Response: {:?}", response);
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

#[cfg(test)]
mod tests {
    use crate::TYPE_A;

    use super::*;

    #[tokio::test]
    async fn test_resolve_async() {
        let res = resolve_async("twitter.com", TYPE_A).await.unwrap();
        println!("{:?}", res);
    }
}
