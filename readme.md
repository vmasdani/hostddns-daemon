# HostDDNS Daemon

HostDDN daemon to send Public IP information every 1 minute.

## Building
1. Install [Rust](https://www.rust-lang.org/learn/get-started)
2. Create a `.env` file
3. Put `UPDATE_URL` to `.env` file
```sh
UPDATE_URL=https://tunnel.hostddns.us/ddns/examplehostddnsupdateurl
```
4. Run `cargo run`

## Cross compiling MUSL
1. Install [cross](https://github.com/rust-embedded/cross)
2. Run
```sh
cross build --target x86_64-unknown-linux-musl --release
```

## Systemd
1. Create file in `/etc/systemd/system/hostddns-daemon.service`. Example service file (might want to change ExecStart & WorkiingDirectory)
```sh
[Unit]
Description=HostDDNS Daemon Service
After=network.target

[Service]
Type=Simple
ExecStart=/root/hostddns-daemon/hostddns-daemon
WorkingDirectory=/root/hostddns-daemon
User=root

[Install]
WantedBy=multi-user.target
```
2. Run service
```sh
sudo systemctl start hostddns-daemon
```

3. Enable service
```sh
sudo systemctl enable hostddns-daemon
```
