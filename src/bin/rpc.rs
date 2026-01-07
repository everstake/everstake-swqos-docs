//This is the minimum working way to send a transaction to Everstake SWQoS service. 
//If you want to improve transaction inclusion - increase priority-fee, introduce your own retry logic
use {
    solana_client::rpc_client::RpcClient,
    solana_sdk::{
        signature::{Signer, read_keypair_file},
        message::Message,
        transaction::Transaction,
        pubkey,
    },
    solana_system_interface::instruction,
    solana_perf::packet::PACKET_DATA_SIZE,
    std::time::Duration,
};

fn main() {
    let solana_client = RpcClient::new("https://api.mainnet-beta.solana.com"); 
    
    // HTTP/1.1 and HTTP/2 both use persistent connections by default.
    // The TCP connection stays open and is reused for subsequent requests,
    // eliminating handshake overhead on each transaction.
    //
    // Using HTTP/2 provides additional benefits:
    // - Multiplexing: multiple requests over a single connection
    // - Header compression (HPACK)
    // - Better performance for high-frequency transaction sending
    //
    // For http:// endpoints, use `http2_prior_knowledge()` to enable HTTP/2 cleartext (h2c).
    // For https:// endpoints, HTTP/2 is negotiated automatically via ALPN.
    //
    // HTTPS example (HTTP/2 negotiated automatically):
    // let everstake_swqos_client = RpcClient::new("https://fra-swqos.everstake.one");
    //
    // HTTP/2 cleartext (h2c) - lowest latency, no encryption overhead:
    let raw_client = reqwest::Client::builder()
        .http2_prior_knowledge() // Required for HTTP/2 over plain HTTP (h2c)
        .default_headers(solana_rpc_client::http_sender::HttpSender::default_headers())
        .timeout(Duration::from_secs(30))
        .pool_idle_timeout(Duration::from_secs(30)) // Keep connection alive for reuse
        .build()
        .expect("Failed to build raw rpc client");

    // TODO: use one of RESOURCES.md Everstake SWQoS RPC Endpoints
    let http_sender = solana_rpc_client::http_sender::HttpSender::new_with_client("http://fra-swqos.everstake.one", raw_client);
    let rpc_client_config = solana_rpc_client::rpc_client::RpcClientConfig::with_commitment(solana_client::rpc_config::CommitmentConfig::confirmed());

    let everstake_swqos_client = RpcClient::new_sender(http_sender, rpc_client_config);

    // TODO: use your keypair file path
    let sender = read_keypair_file("~/.config/solana/id.json").unwrap(); 
    
    // TODO: set tip payment account from `RESOURCES.md`
    let tip_pubkey = pubkey!("J4cL8c22KNLHwheuWxK1SCYBWASWPGhEi6xvcGyf6o3S"); 

    // For demonstration we set receiver equal to sender (self-transfer)
    let receiver = sender.pubkey(); 

    // This instruction is mandatory. Without it, Everstake SWQoS will skip your transaction.
    // First: transfer tip to `tip_pubkey`. This is necessary for your transaction to be processed by Everstake SWQoS.
    let tip_instruction = instruction::transfer(&sender.pubkey(), &tip_pubkey, 500_000); 
    // Second: a self-transfer from sender to receiver (same pubkey) to demonstrate multiple instructions
    let self_transfer = instruction::transfer(&sender.pubkey(), &receiver, 1_000);
    let message = Message::new(&[tip_instruction, self_transfer], Some(&sender.pubkey()));
    let recent_blockhash = solana_client.get_latest_blockhash().unwrap();
    let transaction = Transaction::new(&[&sender], message, recent_blockhash);

    // Validate transaction size before sending
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
        Ok(signature) => println!("Transaction with signature: {} was sent successfully", signature),
        Err(err) => eprintln!("Error while sending transaction: {}", err),
    }
}
