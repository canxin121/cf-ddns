use anyhow::Result;
use cf_api::{delete_dns::delete_dns_record, list_dns::list_dns_records, list_zones::list_zones};
use config::Config;

pub mod cf_api;
pub mod config;
pub mod ip;

pub fn create_client(token: &str) -> reqwest::blocking::Client {
    reqwest::blocking::Client::builder()
        .default_headers(
            std::iter::once((
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", token).parse().unwrap(),
            ))
            .collect(),
        )
        .build()
        .unwrap()
}

pub fn delete_old_dns_records(client: &reqwest::blocking::Client, config: &Config) -> Result<()> {
    let zones = list_zones(client)?;
    let public_ips = ip::get_public_ipaddrs();
    for zone in zones {
        let dns_records = list_dns_records(client, &zone.id)?;
        for dns_record in dns_records {
            // delete old dns records by name
            if dns_record.name.starts_with(&format!("[{}]", config.device)) {
                match delete_dns_record(client, &zone.id, &dns_record.id) {
                    Ok(_) => println!("Deleted dns record: {:?}", dns_record),
                    Err(e) => eprintln!("Failed to delete dns record: {:?}", e),
                }
            }

            // delete old dns records by ip
            if public_ips.iter().find(|i| dns_record == **i).is_some() {
                if !dns_record.name.starts_with(&format!("[{}]", config.device)) {
                    match delete_dns_record(client, &zone.id, &dns_record.id) {
                        Ok(_) => println!("Deleted dns record: {:?}", dns_record),
                        Err(e) => eprintln!("Failed to delete dns record: {:?}", e),
                    }
                }
            }
        }
    }
    Ok(())
}
