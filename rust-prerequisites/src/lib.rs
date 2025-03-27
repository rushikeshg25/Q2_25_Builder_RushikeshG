mod programs;

#[cfg(test)]
mod tests {
    use crate::programs::turbin3_prereq::{CompleteArgs, TurbinePrereqProgram};
    use solana_client::rpc_client::RpcClient;
    use solana_program::hash::hash;
    use solana_program::system_instruction::transfer;
    use solana_sdk::message::Message;
    use solana_sdk::signature::{read_keypair_file, Keypair, Signer};
    use solana_sdk::transaction::Transaction;
    use solana_sdk::{bs58, system_program};
    use std::io::{self, BufRead};

    const RPC_URL: &str = "https://api.devnet.solana.com";

    #[test]
    fn keygen() {
        let key_pair = Keypair::new();
        println!("Pubkey: {}", key_pair.pubkey().to_string());
        println!("{:?}", key_pair.to_bytes());
    }

    #[test]
    fn airdrop() {
        let key_pair = read_keypair_file("temp_wallet.json").expect("Keypair file does not exist");
        let client = RpcClient::new(RPC_URL);
        match client.request_airdrop(&key_pair.pubkey(), 2_000_000_000u64) {
            Ok(s) => {
                println!("Success! Check out your TX here:");

                println!(
                    "https://explorer.solana.com/tx/{}?cluster=devnet",
                    s.to_string()
                );
            }

            Err(e) => println!("Oops, something went wrong: {}", e.to_string()),
        };
    }

    #[test]
    fn transfer_solana() {
        let key_pair = read_keypair_file("temp_wallet.json").expect("Keypair file does not exist");
        let pubkey = key_pair.pubkey();
        let message_bytes = b"I verified my Solana Keypair";
        let sig = key_pair.sign_message(message_bytes);
        let sig_hashed = hash(sig.as_ref());
        match sig.verify(&pubkey.to_bytes(), &sig_hashed.to_bytes()) {
            true => println!("Signature verified"),
            false => println!("Verification failed"),
        }
        let to_keypair =
            read_keypair_file("Turbin3-wallet.json").expect("Keypair file does not exist");
        let to_pubkey = to_keypair.pubkey();
        let rpc_client = RpcClient::new(RPC_URL);
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get latest blockhash");
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&key_pair.pubkey(), &to_pubkey, 1_000_000)],
            Some(&key_pair.pubkey()),
            &vec![&key_pair],
            recent_blockhash,
        );
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }

    #[test]
    fn transfer_all() {
        let rpc_client = RpcClient::new(RPC_URL);
        let from_keypair =
            read_keypair_file("temp_wallet.json").expect("Keypair file does not exist");
        let from_pubkey = from_keypair.pubkey();
        let to_keypair =
            read_keypair_file("Turbin3-wallet.json").expect("Keypair file does not exist");
        let to_pubkey = to_keypair.pubkey();
        let balance = rpc_client.get_balance(&from_pubkey).unwrap();
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get latest blockhash");
        let message = Message::new_with_blockhash(
            &[transfer(&from_pubkey, &to_pubkey, balance)],
            Some(&from_pubkey),
            &recent_blockhash,
        );
        let fee = rpc_client
            .get_fee_for_message(&message)
            .expect("Failed to get fee");
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&from_pubkey, &to_pubkey, balance - fee)],
            Some(&from_pubkey),
            &vec![&from_keypair],
            recent_blockhash,
        );
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }

    #[test]
    fn base58_to_wallet() {
        println!("Input your private key as base58:");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        println!("Your wallet file is:");
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("{:?}", wallet);
    }

    #[test]
    fn enroll() {
        let rpc_client = RpcClient::new(RPC_URL);
        let signer = read_keypair_file("Turbin3-wallet.json").expect("Couldn't find wallet file");
        let prereq = TurbinePrereqProgram::derive_program_address(&[
            b"prereq",
            signer.pubkey().to_bytes().as_ref(),
        ]);
        let args = CompleteArgs {
            github: b"rushikeshg25".to_vec(),
        };
        let blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");
        let transaction = TurbinePrereqProgram::complete(
            &[&signer.pubkey(), &prereq, &system_program::id()],
            &args,
            Some(&signer.pubkey()),
            &[&signer],
            blockhash,
        );
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }
}
