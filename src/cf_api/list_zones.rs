use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{Message, CLIENT};

pub fn list_zones() -> Result<Vec<Zone>> {
    let url = "https://api.cloudflare.com/client/v4
/zones";
    let response = CLIENT.get().unwrap().get(url).send()?;
    let text = response.text()?;
    let zones: ListZones = serde_json::from_str(&text)?;
    if zones.success {
        return Ok(zones.result);
    } else {
        return Err(anyhow::anyhow!("Failed to list zones: {:?}", zones.errors));
    }
}

#[derive(Eq, Hash, Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListZones {
    pub errors: Vec<Message>,
    // pub messages: Vec<Message>,
    pub success: bool,
    // #[serde(rename = "result_info")]
    // pub result_info: ListZoneInfo,
    pub result: Vec<Zone>,
}

#[derive(Eq, Hash, Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListZoneInfo {
    pub count: Option<i64>,
    pub page: Option<i64>,
    #[serde(rename = "per_page")]
    pub per_page: Option<i64>,
    #[serde(rename = "total_count")]
    pub total_count: Option<i64>,
}

#[derive(Eq, Hash, Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Zone {
    // pub account: Account,
    // #[serde(rename = "activated_on")]
    // pub activated_on: String,
    // #[serde(rename = "created_on")]
    // pub created_on: String,
    // #[serde(rename = "development_mode")]
    // pub development_mode: i64,
    pub id: String,
    // pub meta: Meta,
    // #[serde(rename = "modified_on")]
    // pub modified_on: String,
    pub name: String,
    // #[serde(rename = "name_servers")]
    // pub name_servers: Vec<String>,
    // #[serde(rename = "original_dnshost")]
    // pub original_dnshost: Option<String>,
    // #[serde(rename = "original_name_servers")]
    // pub original_name_servers: Vec<String>,
    // #[serde(rename = "original_registrar")]
    // pub original_registrar: String,
    // pub owner: Owner,
    // pub paused: Option<bool>,
    // pub status: Option<String>,
    // #[serde(rename = "type")]
    // pub type_field: Option<String>,
    // #[serde(rename = "vanity_name_servers")]
    // #[serde(default)]
    // pub vanity_name_servers: Vec<String>,
}

#[derive(Eq, Hash, Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Eq, Hash, Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    #[serde(rename = "cdn_only")]
    pub cdn_only: Option<bool>,
    #[serde(rename = "custom_certificate_quota")]
    pub custom_certificate_quota: Option<i64>,
    #[serde(rename = "dns_only")]
    pub dns_only: Option<bool>,
    #[serde(rename = "foundation_dns")]
    pub foundation_dns: Option<bool>,
    #[serde(rename = "page_rule_quota")]
    pub page_rule_quota: Option<i64>,
    #[serde(rename = "phishing_detected")]
    pub phishing_detected: Option<bool>,
    pub step: Option<i64>,
}

#[derive(Eq, Hash, Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Owner {
    pub id: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
}
