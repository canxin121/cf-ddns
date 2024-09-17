use std::collections::HashMap;

use anyhow::Result;
use cf_api::{delete_dns::delete_dns_record, list_dns::list_dns_records, list_zones::list_zones};
use config::Config;
use ip::{del_ip_cache, get_ip_difference, has_ip_cache};

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

pub fn re_init_cfddns(config: Option<Config>) -> Config {
    // delete ip cache and run with new ip
    let config = config.unwrap_or_else(|| Config::load());
    let client = create_client(&config.token);
    if has_ip_cache() {
        del_ip_cache();
    }

    match delete_old_dns_records(&client, &config) {
        Ok(_) => println!("Deleted old dns records"),
        Err(e) => eprintln!("Failed to delete old dns records: {:?}", e),
    }

    apply_ip_differences(get_ip_difference(), &config);
    config
}

pub fn apply_ip_differences(ip_differences: Vec<ip::IpDifference>, config: &Config) {
    if ip_differences.is_empty() {
        return;
    }
    let client = create_client(&config.token);

    let zones = list_zones(&client).unwrap_or_default();
    let mut dns_records = HashMap::new();
    for zone in &zones {
        let records = list_dns_records(&client, &zone.id).unwrap_or_default();
        if !records.is_empty() {
            dns_records.insert(zone.name.to_string(), records);
        }
    }

    for ip_difference in ip_differences {
        match ip_difference.apply(&client, &config, &zones, &dns_records) {
            Ok(_) => {
                println!("Succeed to apply ip_difference [{:?}]", ip_difference);
            }
            Err(e) => {
                eprintln!(
                    "Failed to apply ip_difference [{:?}]: {:?}",
                    ip_difference, e
                );
            }
        }
    }
}

pub fn delete_old_dns_records(client: &reqwest::blocking::Client, config: &Config) -> Result<()> {
    let zones = list_zones(client)?;
    let public_ips = ip::get_public_ipaddrs();
    for zone in zones {
        let dns_records = list_dns_records(client, &zone.id)?;
        println!("zone: {:?}, dns_records: {:?}", zone, dns_records);
        for dns_record in dns_records {
            // delete old dns records by name
            if let Some(comment) = &dns_record.comment {
                if comment.starts_with(&format!("[{}]", config.device)) {
                    match delete_dns_record(client, &zone.id, &dns_record.id) {
                        Ok(_) => println!("Deleted dns record: {:?}", dns_record),
                        Err(e) => eprintln!("Failed to delete dns record: {:?}", e),
                    }
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
