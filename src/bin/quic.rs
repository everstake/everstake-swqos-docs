//This is the minimum working way to send a transaction to Everstake SWQoS Quic service. 
//If you want to improve transaction inclusion - increase priority-fee, introduce your own retry logic
use std::sync::Arc;
use anyhow::{anyhow, Context, Result};
use quinn::{Connection, Endpoint};
use quinn::crypto::rustls::QuicClientConfig;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    message::Message,
    signature::{read_keypair_file, Keypair, Signer},
    transaction::Transaction,
};
use solana_system_interface::instruction;
use solana_tls_utils::{new_dummy_x509_certificate, SkipServerVerification};

const ALPN_SWQOS_TX_PROTOCOL: &[&[u8]] = &[b"solana-tpu"];

pub struct QuicClient {
    _endpoint: Endpoint,
    connection: Connection,
}

//Establish a connection to Everstake SWQoS Quic Endpoint
impl QuicClient {
    pub async fn connect(addr: &str, keypair: &Keypair) -> Result<Self> {
        let (cert, key) = new_dummy_x509_certificate(keypair);

        let mut crypto = rustls::ClientConfig::builder()
            .dangerous()
            .with_custom_certificate_verifier(SkipServerVerification::new())
            .with_client_auth_cert(vec![cert], key)
            .context("failed to configure client certificate")?;

        crypto.alpn_protocols = ALPN_SWQOS_TX_PROTOCOL.iter().map(|p| p.to_vec()).collect();

        let client_crypto = QuicClientConfig::try_from(crypto)
            .context("failed to convert rustls config into quinn crypto config")?;
        let client_config = quinn::ClientConfig::new(Arc::new(client_crypto));

        let mut endpoint = Endpoint::client("0.0.0.0:0".parse()?)?;
        endpoint.set_default_client_config(client_config.clone());
        
        let connection = endpoint
            .connect_with(client_config, addr.parse()?, "everstake_swqos")?
            .await?;

        Ok(Self { _endpoint: endpoint, connection })
    }

    // Send a transaction via quic using a unidirectional stream
    pub async fn send_transaction(&self, transaction: &Transaction) -> Result<()> {
        let signature = transaction.signatures.first().expect("Transaction must have at least one signature");
        let serialized_tx = bincode::serialize(transaction)?;

        let mut send_stream = self.connection.open_uni().await?;
        send_stream.write_all(&serialized_tx).await?;
        send_stream.finish()?;

        println!("Transaction {signature:?} has been sent");
        Ok(())
    }
}

// Create simple stransaction
fn create_transaction(rpc_client: &RpcClient, signer: &Keypair) -> Result<Transaction> {
    let receiver = signer.pubkey();

    let self_transfer = instruction::transfer(&signer.pubkey(), &receiver, 1_000);
    let message = Message::new(&[self_transfer], Some(&signer.pubkey()));
    let recent_blockhash = rpc_client
        .get_latest_blockhash()
        .context("failed to fetch recent blockhash")?;

    let transaction = Transaction::new(&[signer], message, recent_blockhash);

    Ok(transaction)
}

#[tokio::main]
async fn main() -> Result<()> {
    // To establish a connection, use the keypair whose pubkey you previously authorized in our service.
    let everstake_swqos_authorized_keypair = read_keypair_file("~/.config/solana/id.json")
        .map_err(|err| anyhow!("failed to read authorized keypair: {err}"))?;
    
    let solana_client = RpcClient::new("https://api.mainnet-beta.solana.com");
    let transaction = create_transaction(&solana_client, &everstake_swqos_authorized_keypair)?;

    // Use the address from RESOURCES.MD
    let everstake_swqos_quic_addr: &str = "64.130.57.62:11809";

    println!("Connecting to QUIC server...");
    let client = QuicClient::connect(
        everstake_swqos_quic_addr,
        &everstake_swqos_authorized_keypair,
    ).await?;

    client.send_transaction(&transaction).await?;

    Ok(())
}