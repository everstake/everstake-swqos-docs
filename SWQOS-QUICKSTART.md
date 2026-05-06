# Everstake SWQoS Quickstart

> This guide walks you through sending a Solana transaction via Everstake SWQoS RPC from scratch.

## What is Everstake SWQoS?

Everstake SWQoS is a high-performance transaction relay for the Solana network. Instead of submitting your transactions to a generic public RPC endpoint, you route them through Everstake's infrastructure, which forwards them directly to current and upcoming block leaders with minimal latency — improving both speed and inclusion rate.

**Why use it?**
- Higher transaction inclusion rate
- Lower latency compared to standard public RPC nodes
- No API key required — just add a small tip to your transaction

---

## Prerequisites

- Rust toolchain installed (via [rustup](https://rustup.rs))
- A funded Solana keypair (default location: `~/.config/solana/id.json`)
- A basic understanding of what a Solana transaction is — if you're new, start with the [Solana docs](https://solana.com/docs)

---

## How it works

```
Your App ──(signed tx)──▶ Everstake SWQoS RPC ──▶ Current & upcoming block leaders
```

1. **Build** a standard Solana transaction that includes a small tip to Everstake.
2. **Submit** it to an Everstake SWQoS endpoint via the standard `sendTransaction` JSON-RPC method.
3. **Done.** Everstake forwards it to network leaders on your behalf.

The tip (minimum 500,000 lamports = 0.0005 SOL) is **mandatory** — it activates priority forwarding. Without it, your transaction will be dropped.

> **No preflight simulation:** Everstake SWQoS does not run preflight checks. Transactions are forwarded as-is. Make sure your transaction is valid before sending.

---

## Step 1 — Add dependencies

Your `Cargo.toml` needs the following:

```toml
[dependencies]
solana-client         = "3.0.0"
solana-sdk            = "3.0.0"
solana-system-interface = { version = "3.0.0", features = ["bincode"] }
solana-perf           = "3.1.5"
solana-rpc-client     = "3.1.5"
reqwest               = "0.12"
bincode               = "1.3"
```

---

## Step 2 — Set up the SWQoS RPC client

You need **two clients**:

| Client | Purpose |
|--------|---------|
| Standard Solana RPC | Read chain state (latest blockhash, account info, etc.) |
| Everstake SWQoS client | Submit transactions |

The Everstake client is a standard `RpcClient` backed by an HTTP/2-enabled `reqwest` client for maximum performance. HTTP/2 multiplexes multiple requests over a single persistent TCP connection, eliminating per-request handshake overhead.

[View in example → `src/bin/rpc.rs` lines 18–48](https://github.com/everstake/everstake-swqos-docs/blob/main/src/bin/rpc.rs#L18-L48)

```rust
// Standard public RPC — used only for reading chain state
let solana_client = RpcClient::new("https://api.mainnet-beta.solana.com");

// HTTP/2 cleartext (h2c) — lowest latency, no TLS overhead
let raw_client = reqwest::Client::builder()
    .http2_prior_knowledge()  // enables HTTP/2 over plain HTTP
    .default_headers(solana_rpc_client::http_sender::HttpSender::default_headers())
    .timeout(Duration::from_secs(30))
    .pool_idle_timeout(Duration::from_secs(30))  // keep connection alive for reuse
    .build()
    .expect("Failed to build raw rpc client");

let http_sender = solana_rpc_client::http_sender::HttpSender::new_with_client(
    "http://main-swqos.everstake.one",  // Main Cloudflare endpoint
    raw_client,
);
let rpc_client_config = solana_rpc_client::rpc_client::RpcClientConfig::with_commitment(
    solana_client::rpc_config::CommitmentConfig::confirmed(),
);

let everstake_swqos_client = RpcClient::new_sender(http_sender, rpc_client_config);
```

**Choosing an endpoint:** Use the **Main Cloudflare** endpoint (`http://main-swqos.everstake.one` / `https://main-swqos.everstake.one`)
**HTTP vs HTTPS:**
- `http://` with `http2_prior_knowledge()` — lowest latency, no TLS overhead (recommended)
- `https://` — HTTP/2 negotiated automatically via ALPN, slight TLS overhead

---

## Step 3 — Load your keypair

[View in example → `src/bin/rpc.rs` line 51](https://github.com/everstake/everstake-swqos-docs/blob/main/src/bin/rpc.rs#L51)

```rust
let sender = read_keypair_file("~/.config/solana/id.json").unwrap();
```

This keypair is both the **transaction signer** and the **fee payer**. It will pay:
- The standard Solana network fee
- The tip to Everstake SWQoS (≥ 500,000 lamports)
- Any SOL moved by your own instructions

---

## Step 4 — Build the transaction

A valid SWQoS transaction must contain **at minimum**:
1. A **tip instruction** — `SystemProgram::transfer` to an Everstake tip address (≥ 500,000 lamports)
2. **Your instructions** — whatever your application needs to do

3. A **memo instruction** — attach an invoice ID, order reference, or any string identifier

### 4a. Tip instruction (mandatory)

[View in example → `src/bin/rpc.rs` lines 59–61](https://github.com/everstake/everstake-swqos-docs/blob/main/src/bin/rpc.rs#L59-L61)

> **Pick a tip address randomly on every transaction.** All tip accounts are writable during execution. If many transactions target the same address simultaneously, they create write-lock contention on that account, which can delay or drop them. Spreading load across all available addresses avoids this.

```rust
use rand::seq::SliceRandom;

const TIP_ACCOUNTS: &[&str] = &[
    "J4cL8c22KNLHwheuWxK1SCYBWASWPGhEi6xvcGyf6o3S",
    "EzuhsszPxRUHBwGPXtKoqCB58EiTJ1QiYA2XrhbUEFbr",
    "7wsUm2VDopGDFyXkyhmgUh9V15QkEvnyqbgUPcagLcw2",
    "Cy3WAM9NdjFG3kXCxXmD17WmtJMBKVpoBXabkSm88Xdt",
    "BEEya88mme6JJ4rgshBR23eiDHmygUii9opUHE3qxnqK",
    "Gq21dPAGVuuZucqBQeCkfbbqoEowL1t88igZekJ93CRu",
    "79HFWkNoPhotXuFYi1ksuK5hE7AUnKasafP6c71hS9sM",
    "Cp4pCm5JjDaZ4gXB8eSjNJvQ8eg7uK6awgjveofrSATz",
    "DMHQ51qK2wChtDEUED54cqzbSLMLGvTygQCv5uLTUmZP",
    "GDnz7cAA7hKEFmDyrk6mz3drybHWc3Gn14y9LCsvvtjE",
];

let tip_str = TIP_ACCOUNTS.choose(&mut rand::thread_rng()).unwrap();
let tip_pubkey: Pubkey = tip_str.parse().unwrap();

// Minimum: 500_000 lamports (0.0005 SOL)
let tip_instruction = instruction::transfer(&sender.pubkey(), &tip_pubkey, 500_000);
```

> Do **not** add the tip address to an Address Lookup Table — pass it as a regular account.

### 4b. Your application instructions

[View in example → `src/bin/rpc.rs` lines 62–63](https://github.com/everstake/everstake-swqos-docs/blob/main/src/bin/rpc.rs#L62-L63)

```rust
// Replace this with your own instructions
let self_transfer = instruction::transfer(&sender.pubkey(), &receiver, 1_000);
```

You can include any number of instructions. The example uses a simple SOL self-transfer for demonstration.

### 4c. Memo instruction

A memo lets you permanently attach a human-readable string to your transaction — useful for invoice numbers, order IDs, or any custom reference. It will be visible in transaction logs on any Solana explorer and is indexed by most RPC providers, making payment reconciliation easy.

[View in example → `src/bin/rpc.rs` lines 65–71](https://github.com/everstake/everstake-swqos-docs/blob/main/src/bin/rpc.rs#L65-L71)

```rust
let memo_data = b"Invoice #12345";  // replace with your unique identifier
let memo_instruction = Instruction {
    program_id: pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"),
    accounts: vec![],
    data: memo_data.to_vec(),
};
```

### 4d. Get the latest blockhash and assemble the transaction

[View in example → `src/bin/rpc.rs` lines 73–75](https://github.com/everstake/everstake-swqos-docs/blob/main/src/bin/rpc.rs#L73-L75)

```rust
// Always fetch the blockhash from the standard RPC, not the SWQoS endpoint
let recent_blockhash = solana_client.get_latest_blockhash().unwrap();

let message = Message::new(
    &[tip_instruction, self_transfer, memo_instruction],
    Some(&sender.pubkey()),
);
let transaction = Transaction::new(&[&sender], message, recent_blockhash);
```

> **Why use the standard RPC for the blockhash?** The blockhash defines the validity window of your transaction (~150 blocks, roughly 60–90 seconds). The SWQoS endpoint is optimized for sending — use a reliable public RPC for reading chain state.

---

## Step 5 — Send the transaction

Before sending, validate the serialized size to ensure it fits within Solana's maximum packet size (1232 bytes). Then submit via the SWQoS client.

[View in example → `src/bin/rpc.rs` lines 77–91](https://github.com/everstake/everstake-swqos-docs/blob/main/src/bin/rpc.rs#L77-L91)

```rust
let serialized_tx = bincode::serialize(&transaction).expect("Failed to serialize transaction");
if serialized_tx.len() > PACKET_DATA_SIZE {
    eprintln!(
        "Transaction size {} exceeds maximum allowed size {}",
        serialized_tx.len(),
        PACKET_DATA_SIZE
    );
    return;
}

match everstake_swqos_client.send_transaction(&transaction) {
    Ok(signature) => println!("Transaction sent! Signature: {}", signature),
    Err(err)      => eprintln!("Error: {}", err),
}
```

---

## Run the full example

Clone the repo, fill in your keypair path and endpoint, then run:

```bash
cargo run --bin rpc
```

**Expected output:**
```
Transaction with signature: <your signature> was sent successfully
```

---

## Quick reference

| Parameter | Value |
|-----------|-------|
| Min tip | 500,000 lamports (0.0005 SOL) |
| Tip instruction position | First in the instruction list |
| Tip address in ALT | Not allowed |
| Default rate limit | 10 transactions per second (TPS) per client |
| Preflight simulation | Disabled — validate your tx before sending |
| Connection protocol | HTTP/2 recommended (h2c for `http://`, ALPN for `https://`) |

Full endpoint list, tip accounts, and min lamports: [RESOURCES.md](https://github.com/everstake/everstake-swqos-docs/blob/main/RESOURCES.md)