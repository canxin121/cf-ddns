use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{CfDnsRecord, Message, CLIENT};

pub fn list_dns_records(zone_id: &str) -> Result<Vec<DnsRecordResult>> {
    let url = format!(
        "https://api.cloudflare.com/client/v4
/zones/{zone_id}/dns_records"
    );
    let response = CLIENT.get().unwrap().get(&url).send()?;
    let text = response.text()?;
    let dns: ListDns = serde_json::from_str(&text)?;
    if dns.success {
        return Ok(dns.result);
    } else {
        return Err(anyhow::anyhow!("Failed to list dns: {:?}", dns.errors));
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListDns {
    pub errors: Vec<Message>,
    // pub messages: Vec<Message>,
    pub success: bool,
    // #[serde(rename = "result_info")]
    // pub result_info: ListDnsInfo,
    pub result: Vec<DnsRecordResult>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListDnsInfo {
    pub count: Option<i64>,
    pub page: Option<i64>,
    #[serde(rename = "per_page")]
    pub per_page: Option<i64>,
    #[serde(rename = "total_count")]
    pub total_count: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DnsRecordResult {
    pub comment: Option<String>,
    pub name: String,
    // pub proxied: bool,
    // pub tags: Vec<Tag>,
    // pub ttl: i64,
    pub content: String,
    #[serde(rename = "type")]
    pub type_field: String,
    // #[serde(rename = "comment_modified_on")]
    // #[serde(default)]
    // pub comment_modified_on: Option<String>,
    // #[serde(rename = "created_on")]
    // pub created_on: String,
    pub id: String,
    // pub meta: Meta,
    // #[serde(rename = "modified_on")]
    // // pub modified_on: String,
    // pub proxiable: bool,
    // #[serde(rename = "tags_modified_on")]
    // #[serde(default)]
    // pub tags_modified_on: Option<String>,
}

impl Into<CfDnsRecord> for DnsRecordResult {
    fn into(self) -> CfDnsRecord {
        CfDnsRecord {
            name: self.name,
            type_field: self.type_field,
            content: self.content,
            comment: self.comment,
            ..Default::default()
        }
    }
}

impl PartialEq<std::net::IpAddr> for DnsRecordResult {
    fn eq(&self, other: &std::net::IpAddr) -> bool {
        self.content == other.to_string()
    }
}

impl PartialEq<std::net::IpAddr> for &DnsRecordResult {
    fn eq(&self, other: &std::net::IpAddr) -> bool {
        self.content == other.to_string()
    }
}

#[cfg(test)]
mod test_dns_eq {
    #[test]
    fn test_dns_eq() {
        use super::DnsRecordResult;
        use std::net::IpAddr;
        let dns_v4 = DnsRecordResult {
            name: "example.com".to_string(),
            content: "192.168.2.1".to_string(),
            type_field: "A".to_string(),
            ..Default::default()
        };
        let ip_v4 = IpAddr::from([192, 168, 2, 1]);
        assert_eq!(dns_v4, ip_v4);

        let dns_v6 = DnsRecordResult {
            name: "example.com".to_string(),
            content: "2409:8a44:987b:36b0:a8e9:2fff:fe00:38".to_string(),
            type_field: "AAAA".to_string(),
            ..Default::default()
        };
        let ip_v6 = IpAddr::from([0x2409, 0x8a44, 0x987b, 0x36b0, 0xa8e9, 0x2fff, 0xfe00, 0x38]);
        assert_eq!(dns_v6, ip_v6);
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    absent: Option<String>,
    contains: Option<String>,
    endwith: Option<String>,
    exact: Option<String>,
    present: Option<String>,
    startwith: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    #[serde(rename = "auto_added")]
    pub auto_added: Option<bool>,
}
