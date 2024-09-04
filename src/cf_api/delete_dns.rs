use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::CLIENT;

pub fn delete_dns_record(zone_id: &str, dns_record_id: &str) -> Result<()> {
    let url = format!(
        "https://api.cloudflare.com/client/v4
/zones/{zone_id}/dns_records/{dns_record_id}"
    );
    let response = CLIENT.get().unwrap().delete(&url).send()?;
    let text = response.text()?;
    let resp: DeleteResultRoot = serde_json::from_str(&text)?;
    if resp.result.id != dns_record_id {
        return Err(anyhow::anyhow!("Delete Dns Errors: {:#?}", resp.result));
    }
    Ok(())
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteResultRoot {
    pub result: DeleteResult,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteResult {
    pub id: String,
}
