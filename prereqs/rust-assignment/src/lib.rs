mod programs;

#[cfg(test)]
mod tests {
    use crate::programs::turbine_prereq::{CompleteArgs, Turbin3PrereqProgram};
    use solana_client::rpc_client::RpcClient;
    use solana_program::{pubkey::Pubkey, system_instruction::transfer};
    use solana_sdk::{
        message::Message,
        signature::{read_keypair_file, Keypair, Signer},
        system_program,
        transaction::Transaction,
    };
    use std::str::FromStr;

    const RPC_URL: &str = "https://api.devnet.solana.com";

    #[test]
    fn keygen() {
        let kp = Keypair::new();
        println!(
            "You have generated a solana wallet:{}",
            kp.pubkey().to_string()
        );
        println!("");
        println!("To save your wallet copy and paste the following and save it JSON file");

        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn airdrop() {
        let keypair = read_keypair_file("dev-wallet.json").expect("couldn't find wallet file");
        let client = RpcClient::new(RPC_URL);

        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(a) => {
                println!("success !! check tx here");
                println!(
                    "https://explorer.solana.com/tx/{}?cluster=devnet",
                    a.to_string()
                );
            }
            Err(e) => println!("Oops ,something wen wronf : {}", e.to_string()),
        }
    }

    #[test]
    fn transfer_sol() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet");
        let to_pubkey = Pubkey::from_str("7yH6pP6VhXoPwjzqHQy6hy8tGZUsPfTJzUdKJessUS2M").unwrap();
        let rpc_client = RpcClient::new(RPC_URL);
        let balance = rpc_client
            .get_balance(&keypair.pubkey())
            .expect("Failed to get Balance");
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("failed to get recent blockhash");
        let message = Message::new_with_blockhash(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
            Some(&keypair.pubkey()),
            &recent_blockhash,
        );
        let fee = rpc_client
            .get_fee_for_message(&message)
            .expect("failed to get fee calculator");
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("failed to send transaction");
        println!(
            "Success! Check out TX here : https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }

    #[test]
    fn enroll() {
        let rpc_client = RpcClient::new(RPC_URL);
        let signer = read_keypair_file("Turbin3_wallet.json").expect("couldn't find file");

        let prereq = Turbin3PrereqProgram::derive_program_address(&[
            b"prereq",
            signer.pubkey().to_bytes().as_ref(),
        ]);

        let args = CompleteArgs {
            github: b"shivendram16053".to_vec(),
        };

        let blockhash = rpc_client
            .get_latest_blockhash()
            .expect("failed to get recent blockhash");

        let transaction = Turbin3PrereqProgram::complete(
            &[&signer.pubkey(), &prereq, &system_program::id()],
            &args,
            Some(&signer.pubkey()),
            &[&signer],
            blockhash,
        );

        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("failed to send transaction");

        println!(
            "Success! check out TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }
}
