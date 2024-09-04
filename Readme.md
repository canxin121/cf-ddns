## CF-DDNS

### usage

put `config.toml` in working dictory

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