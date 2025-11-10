# RPC
**EverSender Tip Payment Accounts:**
- `J4cL8c22KNLHwheuWxK1SCYBWASWPGhEi6xvcGyf6o3S`
- `EzuhsszPxRUHBwGPXtKoqCB58EiTJ1QiYA2XrhbUEFbr`
- `7wsUm2VDopGDFyXkyhmgUh9V15QkEvnyqbgUPcagLcw2`
- `Cy3WAM9NdjFG3kXCxXmD17WmtJMBKVpoBXabkSm88Xdt`
- `BEEya88mme6JJ4rgshBR23eiDHmygUii9opUHE3qxnqK`
- `Gq21dPAGVuuZucqBQeCkfbbqoEowL1t88igZekJ93CRu`
- `79HFWkNoPhotXuFYi1ksuK5hE7AUnKasafP6c71hS9sM`
- `Cp4pCm5JjDaZ4gXB8eSjNJvQ8eg7uK6awgjveofrSATz`
- `DMHQ51qK2wChtDEUED54cqzbSLMLGvTygQCv5uLTUmZP`
- `GDnz7cAA7hKEFmDyrk6mz3drybHWc3Gn14y9LCsvvtjE`

**EverSender RPC Endpoints**
- **Main Cloudflare**
  - `https://main-swqos.everstake.one`
  - `http://main-swqos.everstake.one`
- **FRA.**
  - `https://fra-swqos.everstake.one`
	- `http://fra-swqos.everstake.one`

- **NY.**
	- `https://ny-swqos.everstake.one`
	- `http://ny-swqos.everstake.one`

- **TYO.**
	- `https://tyo-swqos.everstake.one`
  - `http://tyo-swqos.everstake.one`

- **AMS.**
	- `https://ams-swqos.everstake.one`
  - `http://ams-swqos.everstake.one`
**Important**: Always use the correct protocol (http/https) for the appropriate port. For example, port 443 only works with HTTPS.

**Min Lamports**
- `500000`

**Special RPC Node Flag**
To send all transactions to a single leader (instead of following the leader schedule), the RPC node must include:
`--rpc-send-transaction-tpu-peer <SocketAddr>`
where `<SocketAddr>` is one of the EverSender QUIC endpoints listed below.

# QUIC
**EverSender Quic Endpoint**
- Frankfurt: `64.130.57.62:11809`
- Tokio: `208.91.107.171:11809`
- Amsterdam: `74.118.140.197:11809`
- NewYork: `64.130.59.154:11809`