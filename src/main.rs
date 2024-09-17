use std::{thread::sleep, time::Duration};

use cf_ddns::{apply_ip_differences, config::Config, ip::get_ip_difference, re_init_cfddns};

fn main() {
    // delete ip cache and run with new ip
    let mut config = re_init_cfddns(None);
    // generate a hash code of the config
    let mut config_hash = config.hash_code();

    loop {
        sleep(Duration::from_secs(config.interval));
        config = Config::load();
        let new_config_hash = config.hash_code();

        if new_config_hash == config_hash {
            let ip_differences = get_ip_difference();
            apply_ip_differences(ip_differences, &config);
        } else {
            config = re_init_cfddns(Some(config));
            config_hash = new_config_hash;
        }
    }
}
