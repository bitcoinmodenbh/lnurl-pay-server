# lnurl-pay-server

Run step first command
#
cargo build
#
Run step second command
#
cargo run
#
move to /usr/local/bin




Run this command for create service
#
nano /etc/systemd/system/lnurlp_server.service
#
past this 
#
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
#




press this key 

control+x


control+y
