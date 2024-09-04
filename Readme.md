## Cf-ddns

### feature
Cf-ddns can automatically sync the local machine's IP address with the DNS records on Cloudflare without affecting the DNS records that are manually configured on the web interface or those automatically configured by other machines running the program. This is achieved by using `comment` in the Cloudflare DNS records and a special device name to distinguish between operations performed by different machines and manual operations.

### usage
Put `config.toml` in working dictory

`config.toml` example
```toml
token = "cf token"
device = "a unique device id"

# Zone1
[[zones]]
name = "my-site.cn"

[[zones.records]]
# required
name = "@"

# below are all optional
# default: all
type = "all"
# default: false
proxied = true
# default: [], you should create the tag mannuly in cf web
tags = ["tag1"]
# default: "[{device}] "
comment = "a comment for this record"
# default: None
ttl = 60

[[zones.records]]
name = "www"
type = "v6"
comment = "a comment for this record"


[[zones.records]]
name = "alist"
type = "v6"

# Zone2
[[zones]]
name = "my-site.com"

[[zones.records]]
name = "@"
```