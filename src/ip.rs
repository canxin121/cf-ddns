use anyhow::Result;
use ipnetwork::IpNetwork;
use local_ip_address::list_afinet_netifas;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::IpAddr, path::Path};

use crate::{
    cf_api::{
        create_dns::create_dns_record, list_dns::DnsRecordResult, list_zones::Zone, CfDnsRecord,
    },
    config::Config,
};

pub fn get_ip_difference() -> Vec<IpDifference> {
    let cache = IpCache::load();
    let current = IpCache::new();
    current.save();
    let differences = cache.different(current);
    differences
}

pub fn has_ip_cache() -> bool {
    Path::new(CACHE_FILE).exists()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_ip_difference() {
        println!("{:?}", get_ip_difference());
        println!("{:?}", get_ip_difference());
        println!("{:?}", get_ip_difference());
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IpCache(pub Vec<IpAddr>);

const CACHE_FILE: &str = "./ip_cache.txt";

impl IpCache {
    pub fn new() -> Self {
        Self(get_public_ipaddrs())
    }

    pub fn save(&self) {
        let ips = self
            .0
            .iter()
            .map(|ip| ip.to_string())
            .collect::<Vec<String>>();
        let ips = ips.join("\n");

        std::fs::write(CACHE_FILE, ips).expect("Failed to write ip_cache.txt");
    }

    pub fn load() -> Self {
        // 不存在则返回默认值
        if !Path::new(CACHE_FILE).exists() {
            return Self::default();
        }

        let ips = std::fs::read_to_string(CACHE_FILE).unwrap_or_default();
        let ips = ips
            .lines()
            .map(|ip| ip.parse().expect("Failed to parse ip_cache.txt"))
            .collect();
        Self(ips)
    }

    pub fn different(self, other: IpCache) -> Vec<IpDifference> {
        let mut differences = Vec::new();

        let self_ips: std::collections::HashSet<_> = self.0.iter().collect();
        let other_ips: std::collections::HashSet<_> = other.0.iter().collect();

        for ip in other_ips.difference(&self_ips) {
            differences.push(IpDifference::Add(**ip));
        }

        for ip in self_ips.difference(&other_ips) {
            differences.push(IpDifference::Remove(**ip));
        }

        differences
    }
}

#[derive(Debug)]
pub enum IpDifference {
    Add(IpAddr),
    Remove(IpAddr),
}

impl IpDifference {
    pub fn apply(
        &self,
        config: &Config,
        zones: &Vec<Zone>,
        dns_records: &HashMap<String, Vec<DnsRecordResult>>,
    ) -> Result<()> {
        match self {
            IpDifference::Add(ip) => {
                for zone_config in &config.zones {
                    for record in &zone_config.records {
                        // 配置的ip和当前ip有关
                        if record.dns_type.related(ip) {
                            // cf 中有这个zone
                            if let Some(zone) = zones.iter().find(|z| z.name == zone_config.name) {
                                let mut record = record.clone();
                                record.comment = format!(
                                    "[{}] {}",
                                    config.device,
                                    record.comment.unwrap_or_default()
                                )
                                .into();

                                match create_dns_record(
                                    &zone.id,
                                    CfDnsRecord::create(ip.clone(), &record),
                                ) {
                                    Ok(_) => {
                                        println!("Created dns record: {:?}", record);
                                    }
                                    Err(_) => {
                                        eprintln!("Failed to create dns record: {:?}", record);
                                    }
                                }
                            }
                        }
                    }
                }

                Ok(())
            }
            IpDifference::Remove(ip) => {
                for zone_config in &config.zones {
                    for record in &zone_config.records {
                        // 配置的ip和当前ip有关
                        if record.dns_type.related(ip) {
                            // cf 中有这个zone
                            if let Some(zone) = zones.iter().find(|z| z.name == zone_config.name) {
                                // cf中有这个zone的dns记录
                                if let Some(records) = dns_records.get(&zone.name) {
                                    // cf中有这个zone的dns记录中有这个ip
                                    if let Some(dns_record) =
                                        records.iter().find(|r| r.content == ip.to_string())
                                    {
                                        // 没有comment或者comment不是以[device]开头的，跳过不删除
                                        if dns_record.comment.is_none()
                                            || dns_record
                                                .comment
                                                .as_ref()
                                                .unwrap()
                                                .starts_with(&format!("[{}]", config.device))
                                        {
                                            continue;
                                        }

                                        match crate::cf_api::delete_dns::delete_dns_record(
                                            &zone.id,
                                            &dns_record.id,
                                        ) {
                                            Ok(_) => {
                                                println!("Deleted dns record: {:?}", dns_record);
                                            }
                                            Err(_) => {
                                                eprintln!(
                                                    "Failed to delete dns record: {:?}",
                                                    dns_record
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                Ok(())
            }
        }
    }
}

pub fn get_public_ipaddrs() -> Vec<IpAddr> {
    let ips = match list_afinet_netifas() {
        Ok(ips) => ips
            .into_iter()
            .filter(|(_name, ip)| !is_private_ip(ip))
            .map(|(_name, ip)| ip)
            .collect(),
        Err(e) => {
            eprintln!("Failed to get public ip addresses: {}", e);
            vec![]
        }
    };
    println!("Get Public IP addresses: {:?}", ips);
    ips
}

fn is_private_ip(ip: &IpAddr) -> bool {
    let ip_networks = [
        IpNetwork::V4("0.0.0.0/8".parse().unwrap()), // 当前网络或本地网络
        IpNetwork::V4("10.0.0.0/8".parse().unwrap()), // 私有网络A类
        IpNetwork::V4("100.64.0.0/10".parse().unwrap()), // 运营商级私有网络
        IpNetwork::V4("127.0.0.0/8".parse().unwrap()), // 本机回环地址
        IpNetwork::V4("169.254.0.0/16".parse().unwrap()), // 链路本地地址（APIPA）
        IpNetwork::V4("172.16.0.0/12".parse().unwrap()), // 私有网络B类
        IpNetwork::V4("192.0.0.0/24".parse().unwrap()), // IANA特殊用途地址
        IpNetwork::V4("192.0.2.0/24".parse().unwrap()), // 文档和测试使用（TEST-NET-1）
        IpNetwork::V4("192.168.0.0/16".parse().unwrap()), // 私有网络C类
        IpNetwork::V4("198.18.0.0/15".parse().unwrap()), // 网络间基准测试地址
        IpNetwork::V4("198.51.100.0/24".parse().unwrap()), // 文档和测试使用（TEST-NET-2）
        IpNetwork::V4("203.0.113.0/24".parse().unwrap()), // 文档和测试使用（TEST-NET-3）
        IpNetwork::V4("224.0.0.0/4".parse().unwrap()), // 组播地址
        IpNetwork::V4("240.0.0.0/4".parse().unwrap()), // 保留地址
        IpNetwork::V4("255.255.255.255/32".parse().unwrap()), // 广播地址
        IpNetwork::V6("::1/128".parse().unwrap()),   // IPv6本机回环地址
        IpNetwork::V6("fc00::/7".parse().unwrap()),  // IPv6私有网络
        IpNetwork::V6("fe80::/10".parse().unwrap()), // IPv6链路本地地址
        IpNetwork::V6("ff00::/8".parse().unwrap()),  // IPv6组播地址
    ];

    ip_networks.iter().any(|net| net.contains(*ip))
}
