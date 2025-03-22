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
cd src  and edit main.rs
```
cd src 
nano main.rs
```

past your info here
```
const DOMAIN: &str = ""; // Replace with your real domain
const LND_URL: &str = ""; // Lightning REST API address
const MACAROON_PATH: &str = ""; // Path to the macaroon file
const TLS_CERT_PATH: &str = ""; // Path to the LND TLS certificate
```
optional step change amount 
```


async fn lnurlp_info() -> impl Responder {
    let callback = format!("{}/.well-known/lnurlp/sats/callback", DOMAIN);
    let metadata = serde_json::to_string(&vec![vec!["text/plain", "Pay 1 to 1000 satoshis"]]).unwrap();
    HttpResponse::Ok().json(json!({
        "callback": callback,
        "maxSendable": 1_000_000, // past max mili sats
        "minSendable": 1_000,     // past min mili sats
        "metadata": metadata,
        "tag": "payRequest"
    }))
}

async fn lnurlp_callback(query: web::Query<CallbackQuery>) -> impl Responder {
    let min_amount = 1_000;  // past min mili sats
    let max_amount = 1_000_000;  // past max milisats

    if query.amount < min_amount || query.amount > max_amount {
        return HttpResponse::BadRequest().body(format!(
            "The amount must be between {} and {} millisatoshis (1 to 1000 satoshis)",
            min_amount, max_amount
        ));
    }



```
Run build command 

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
```
control+x


y


Enter


```

install caddy

```
sudo apt install -y debian-keyring debian-archive-keyring apt-transport-https
curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/gpg.key' | sudo gpg --dearmor -o /usr/share/keyrings/caddy-stable-archive-keyring.gpg
curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/debian.deb.txt' | sudo tee /etc/apt/sources.list.d/caddy-stable.list
sudo apt update
sudo apt install -y caddy
caddy version
```

start && enable caddy
```

sudo systemctl start caddy

sudo systemctl enable caddy

sudo systemctl status caddy

```

get cert 
```
apt-get install certbot -y
certbot certonly --standalone --agree-tos --register-unsafely-without-email -d yourdomain.com
```
edit caddy
```sudo nano /etc/caddy/Caddyfile```

```
yourdomain.com {
    tls /etc/letsencrypt/live/yourdomain.com/fullchain.pem /etc/letsencrypt/live/yourdomain.com/privkey.pem
    reverse_proxy 127.0.0.1:8000
}
```
press this key 
```
control+x


y


Enter


```
reload and restart caddy

```
sudo systemctl reload caddy
sudo systemctl restart caddy
```
#
#
everything is ready
#
#
