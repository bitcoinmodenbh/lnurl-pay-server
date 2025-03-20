lnurl-pay-server

install rust

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
```
source $HOME/.cargo/env
```
install openssl
```
sudo apt-get update
sudo  apt-get install build-essential openssl libssl-dev

```
cd to directory
```
cd lnurl-pay-server
```

Run step first command

```
cargo build --release

```
open your build directory
```
cd /lnurl-pay-server/target/release
```
Run step second command for move to /usr/local/bin

```
mv lnurl_pay /usr/local/bin

```


Run this command for create service
```
nano /etc/systemd/system/lnurlp_server.service
```
past this 

```
[Unit]
Description=LNURL-Pay Server
After=network.target

[Service]
ExecStart=/usr/local/bin/lnurlp_server
Restart=always
User=root
WorkingDirectory=/root/lnurl_pay

[Install]
WantedBy=multi-user.target
```





press this key 

control+x
#

y

#
Enter
#




