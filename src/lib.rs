use anyhow::Result;
use cf_api::{delete_dns::delete_dns_record, list_dns::list_dns_records, list_zones::list_zones};
use config::Config;

pub mod cf_api;
pub mod config;
pub mod ip;

pub fn delete_old_dns_records(config: &Config) -> Result<()> {
    let zones = list_zones()?;
    let public_ips = ip::get_public_ipaddrs();
    for zone in zones {
        let dns_records = list_dns_records(&zone.id)?;
        for dns_record in dns_records {
            if public_ips.iter().find(|i| dns_record == **i).is_some() {
                // 这条dns记录是本地的
                // 如果这个记录不是以[device]开头，则删除
                if !dns_record.name.starts_with(&format!("[{}]", config.device)) {
                    match delete_dns_record(&zone.id, &dns_record.id) {
                        Ok(_) => println!("Deleted dns record: {:?}", dns_record),
                        Err(e) => eprintln!("Failed to delete dns record: {:?}", e),
                    }
                }
            }
        }
    }
    Ok(())
}
