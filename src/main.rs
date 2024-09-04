use std::{collections::HashMap, thread::sleep, time::Duration};

use cf_ddns::{
    cf_api::{init_client, list_dns::list_dns_records, list_zones::list_zones},
    config::Config,
    delete_old_dns_records,
    ip::{get_ip_difference, has_ip_cache},
};

fn main() {
    let config = Config::load();
    init_client(&config.token);
    if !has_ip_cache() {
        match delete_old_dns_records(&config) {
            Ok(_) => println!("Deleted old dns records"),
            Err(e) => eprintln!("Failed to delete old dns records: {:?}", e),
        }
    }

    loop {
        let ip_differences = get_ip_difference();
        if !ip_differences.is_empty() {
            let zones = list_zones().unwrap_or_default();
            let mut dns_records = HashMap::new();
            for zone in &zones {
                let records = list_dns_records(&zone.id).unwrap_or_default();
                if !records.is_empty() {
                    dns_records.insert(zone.name.to_string(), records);
                }
            }

            for ip_difference in ip_differences {
                match ip_difference.apply(&config, &zones, &dns_records) {
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
        sleep(Duration::from_secs(config.interval));
    }
}
