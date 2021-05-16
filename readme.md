# HostDDNS Daemon

HostDDN daemon to send Public IP information every 1 minute.

## Building
1. Install [Rust](https://www.rust-lang.org/learn/get-started)
2. Create a `.env` file
3. Put `UPDATE_URL` to `.env` file
```sh
UPDATE_URL=https://tunnel.hostddns.us/ddns/examplehostddnsupdateurl
```
3. Run `cargo run`

## Cross compiling MUSL
1. Install [cross](https://github.com/rust-embedded/cross)
2. Run
```sh
cross build x86_64-unknown-linux-musl --release
```