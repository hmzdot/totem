# totem

A [STUN](https://en.wikipedia.org/wiki/STUN) server written in Rust.

It is **NOT** intended for production.

## Setup

Requires Docker and Docker compose for network setup.
Otherwise it's hard to simulate a private and a public network

```bash
git clone git@github.com:hmzdot/totem.git
cd totem

docker compose up
```

Docker compose file, sets up two networks (private-net and public-net), then
runs server in the public-net and client in the private-net.

Client connects to NAT to reach out to server and server responds client's IP
address back to client through NAT.
