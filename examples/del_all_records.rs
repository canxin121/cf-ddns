use cf_ddns::{
    cf_api::{delete_dns::delete_dns_record, init_client},
    config::Config,
};

fn main() {
    let config = Config::load();
    init_client(&config.token);

    let zones = cf_ddns::cf_api::list_zones::list_zones().unwrap();
    for zone in zones {
        let dns_vec = cf_ddns::cf_api::list_dns::list_dns_records(&zone.id).unwrap();
        for dns in dns_vec {
            delete_dns_record(&zone.id, &dns.id).unwrap();
        }
    }
}
