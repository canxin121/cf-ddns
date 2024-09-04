use std::{
    hash::{DefaultHasher, Hash, Hasher},
    net::IpAddr,
};

use serde::{Deserialize, Serialize};

#[derive(Default, Hash, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub device: String,
    pub token: String,
    #[serde(default = "default_interval")]
    pub interval: u64,
    #[serde(default)]
    pub zones: Vec<ZoneConfig>,
}

fn default_interval() -> u64 {
    60
}

impl Config {
    pub fn load() -> Self {
        let config = std::fs::read_to_string("config.toml").expect("Failed to read ./config.toml");
        toml::from_str(&config).unwrap()
    }

    pub fn hash_code(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

#[derive(Default, Hash, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ZoneConfig {
    pub name: String,
    pub records: Vec<DnsRecordConfig>,
}

#[derive(Default, Hash, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DnsType {
    #[serde(rename = "all")]
    #[default]
    All,
    #[serde(rename = "v4")]
    V4,
    #[serde(rename = "v6")]
    V6,
}

impl DnsType {
    pub fn related(&self, ip: &IpAddr) -> bool {
        match self {
            DnsType::All => true,
            DnsType::V4 => ip.is_ipv4(),
            DnsType::V6 => ip.is_ipv6(),
        }
    }
}

#[derive(Default, Hash, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DnsRecordConfig {
    pub name: String,
    #[serde(default)]
    #[serde(rename = "type")]
    pub dns_type: DnsType,
    #[serde(default)]
    pub comment: Option<String>,
    #[serde(default)]
    pub proxied: bool,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub ttl: Option<i64>,
}

#[test]
fn test_serialization_deserialization() {
    // Create a sample DnsRecord
    let record = DnsRecordConfig {
        name: "example.com".to_string(),
        dns_type: DnsType::V4,
        comment: Some("This is a comment".to_string()),
        proxied: true,
        tags: vec!["tag1".to_string(), "tag2".to_string()],
        ttl: Some(3600),
    };

    // Create a sample Zone
    let zone = ZoneConfig {
        name: "zone1".to_string(),
        records: vec![record.clone()],
    };

    // Create a sample Config
    let config = Config {
        token: "your_cf_token".to_string(),
        zones: vec![zone.clone()],
        ..Default::default()
    };

    let serialized = toml::to_string_pretty(&config).unwrap();
    println!("Serialized Config:\n{}", serialized);

    let deserialized: Config = toml::from_str(&serialized).unwrap();
    println!("Deserialized Config:\n{:#?}", deserialized);
}
