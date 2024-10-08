use std::net::IpAddr;

use serde::{Deserialize, Serialize};

use crate::config::DnsRecordConfig;

pub mod create_dns;
pub mod delete_dns;
pub mod list_dns;
pub mod list_zones;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CfDnsRecord {
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub content: String,
    #[serde(default)]
    pub comment: Option<String>,
    #[serde(default)]
    pub proxied: Option<bool>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub ttl: Option<i64>,
}

impl CfDnsRecord {
    pub fn create(ip: IpAddr, dns_config: &DnsRecordConfig) -> Self {
        Self {
            name: dns_config.name.clone(),
            type_field: match ip {
                IpAddr::V4(_) => "A",
                IpAddr::V6(_) => "AAAA",
            }
            .to_string(),
            content: ip.to_string(),
            comment: dns_config.comment.clone(),
            proxied: Some(dns_config.proxied),
            tags: dns_config.tags.clone(),
            ttl: dns_config.ttl,
            ..Default::default()
        }
    }
}

#[derive(Eq, Hash, Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub code: i64,
    pub message: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DnsOperationResponse {
    success: bool,
    errors: Vec<Message>,
}
