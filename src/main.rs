use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash as _, Hasher},
    thread::sleep,
    time::Duration,
};

use cf_ddns::{
    apply_ip_differences,
    cf_api::{list_dns::list_dns_records, list_zones::list_zones},
    config::Config,
    create_client, delete_old_dns_records,
    ip::{get_ip_difference, has_ip_cache},
    re_init_cfddns,
};

fn main() {
    // delete ip cache and run with new ip
    let config = re_init_cfddns(None);
    // generate a hash code of the config
    let mut config_hash = config.hash_code();

    loop {
        sleep(Duration::from_secs(config.interval));
        // 加载新的配置
        let config = Config::load();
        let new_config_hash = config.hash_code();

        if new_config_hash == config_hash {
            let ip_differences = get_ip_difference();
            apply_ip_differences(ip_differences, &config);
        } else {
            re_init_cfddns(Some(config));
            config_hash = new_config_hash;
        }
    }
}
