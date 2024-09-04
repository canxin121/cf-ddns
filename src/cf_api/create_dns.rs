use super::{CfDnsRecord, DnsOperationResponse};
use anyhow::Result;

pub fn create_dns_record(
    client: &reqwest::blocking::Client,
    zone_id: &str,
    cf_dns_record: CfDnsRecord,
) -> Result<()> {
    let url = format!(
        "https://api.cloudflare.com/client/v4
/zones/{zone_id}/dns_records"
    );
    let response = client
        .post(&url)
        .body(serde_json::to_string(&cf_dns_record)?)
        .send()?;
    let text = response.text()?;
    let resp: DnsOperationResponse = serde_json::from_str(&text)?;
    if !resp.success {
        return Err(anyhow::anyhow!("Create Dns Errors: {:#?}", resp.errors));
    }
    Ok(())
}
