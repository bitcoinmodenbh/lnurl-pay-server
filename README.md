lnurl-pay-server

install rust

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```



install openssl
```
sudo apt-get update
sudo  apt-get install openssl libssl-dev
```
Run step first command

```
cargo build
```

Run step second command

```
cargo run
```

```
move to /usr/local/bin

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




