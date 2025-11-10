//This is the minimum working way to send a transaction to EverSender service. 
//If you want to improve transaction inclusion - increase priority-fee, introduce your own retry logic
use {
    solana_client::{rpc_client::RpcClient},
    solana_sdk::{
        system_instruction,
        signature::{Signer, read_keypair_file},
        message::Message,
        transaction::Transaction,
        pubkey,
    },
};

fn main() {
    let solana_client = RpcClient::new("https://api.mainnet-beta.solana.com"); 

    // TODO: use one of RESOURCES.md EverSender RPC Endpoints
    let ever_sender_client = RpcClient::new("https://fra-swqos.everstake.one");

    // TODO: use your keypair file path
    let sender = read_keypair_file("~/.config/solana/id.json").unwrap(); 
    
    // TODO: set tip payment account from `RESOURCES.md`
    let tip_pubkey = pubkey!("J4cL8c22KNLHwheuWxK1SCYBWASWPGhEi6xvcGyf6o3S"); 

    // For demonstration we set receiver equal to sender (self-transfer)
    let receiver = sender.pubkey(); 

    // This instruction is mandatory. Without it, EverSender will skip your transaction.
    // First: transfer tip to `tip_pubkey`. This is necessary for your transaction to be processed by EverSender.
    let tip_instruction = system_instruction::transfer(&sender.pubkey(), &tip_pubkey, 500_000); 
    // Second: a self-transfer from sender to receiver (same pubkey) to demonstrate multiple instructions
    let self_transfer = system_instruction::transfer(&sender.pubkey(), &receiver, 1_000);
    let message = Message::new(&[tip_instruction, self_transfer], Some(&sender.pubkey()));
    let recent_blockhash = solana_client.get_latest_blockhash().unwrap();
    let transaction = Transaction::new(&[&sender], message, recent_blockhash);


    match ever_sender_client.send_transaction(&transaction) {
        Ok(signature) => println!("Transaction with signature: {} was sent successfully", signature),
        Err(err) => eprintln!("Error while sending transaction: {}", err),
    }
}
