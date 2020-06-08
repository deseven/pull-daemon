## pull-daemon
A simple daemon that listens to http query `GET/POST /pull` on the desired port to execute a set of scripts from `update-repos.d` directory. Can be used as a very simple CI system if you don't need fancy stuff.

### compiling
`rustc pd.rs && strip pd`

### running
Use the included systemd unit.

### installing pre-compiled binary (linux x86_64) with default configuration
`bash <(curl -s https://d7.wtf/pd/i)`
