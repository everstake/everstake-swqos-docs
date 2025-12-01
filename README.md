# Everstake SWQoS Client

Everstake SWQoS is a low-latency transaction relay that supports two submission modes:
- **RPC**: Submit signed Solana transactions via a standard JSON-RPC interface.
- **QUIC**: Submit Solana transactions via a direct, low-latency QUIC connection.

This repository documents both modes and provides runnable examples for each.

---

## RPC Path

### What is it?
The Everstake SWQoS RPC endpoint is a proxy for sending signed Solana transactions. It speaks standard Solana JSON-RPC and forwards your transactions to current and future leaders for reliable delivery.

**Key points:**
 - No API key required.
 - Your transaction must include a “transfer” instruction with [Min Lamports](RESOURCES.md) to one of these Pubkeys: [Tip Payment Accounts](RESOURCES.md). Without this instruction, your transaction will be dropped.
 - The endpoint for the Everstake SWQoS RPC is listed in [RPC Endpoints](RESOURCES.md).

### How it works
1. You build and sign a Solana transaction in your client.
2. You include a tip instruction (a simple `SystemProgram::transfer`) paying lamports to a designated Everstake SWQoS tip account [Tip Payment Accounts](RESOURCES.md).
3. You submit the transaction via a standard `sendTransaction` call to the Everstake SWQoS RPC endpoint [RPC Endpoints](RESOURCES.md).
4. Everstake SWQoS forwards the transaction for fast, reliable delivery.

### Quick start: Transaction example (Rust)
This repo includes a minimal Rust binary (`rpc.rs`) that:
- Loads your keypair.
- Builds a transaction that tips the Everstake SWQoS tip account and transfers 1000 lamports to yourself.
- Sends the transaction via an RPC URL.

**Run the RPC example:**
```bash
cargo run --bin rpc
```

**Expected output:**
- `Transaction with signature: "your tx signature" was sent successfully`

**Important:**
- For production use, you must set the real Everstake SWQoS RPC endpoint and the real tip Pubkey in the example code.

---

## QUIC Path

### What is it?
The Everstake SWQoS QUIC endpoint offers a direct, low-latency path for submitting signed Solana transactions. It uses the QUIC protocol to stream transactions directly to Everstake SWQoS relay, which then forward them to network leaders. This path is ideal for latency-sensitive applications.

**Key points:**
 - No API key required.
 - The QUIC path provides lower latency compared to RPC.
 - You must authorize your pubkey to connect to Everstake SWQoS via Quic.

 **Connection Rate Limit:** Each authorized client is limited to 8 QUIC connections per minute. Please ensure your integration respects this limit to avoid connection errors or temporary blocks.

### How it works
1. You build and sign a Solana transaction in your client.
3. You establish a QUIC connection to an [Everstake SWQoS QUIC](RESOURCES.md) endpoint after we whitelist your pubkey. 
4. You send the raw, serialized transaction over the QUIC stream.
5. Everstake SWQoS forwards the transaction for fast, reliable delivery.

### Quick start: Transaction example (Rust)
This repo includes a minimal Rust binary (`quic.rs`) that demonstrates the QUIC path:
- Loads your keypair.
- Builds a simple transaction.
- Sends the transaction via a QUIC connection.

**Run the QUIC example:**
```bash
cargo run --bin quic
```

**Important:**
- The RPC node must include a special flag to send all transactions to a single leader instead of following the leader schedule:
  --rpc-send-transaction-tpu-peer `SocketAddr`, where `SocketAddr` - [Everstake SWQoS QUIC endpoint](RESOURCES.md)
  For RPC version 2.3 and above the --use-connection-cache flag must also be specified, since the new TPU client is not compatible with the SWQoS connection.
- For production use, you must configure the correct Everstake SWQoS QUIC endpoint in the example code.
- You must authorize your pubkey to connect to Everstake SWQoS via Quic. 