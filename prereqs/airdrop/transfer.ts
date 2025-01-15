import {
  Keypair,
  LAMPORTS_PER_SOL,
  sendAndConfirmTransaction,
  SystemProgram,
  PublicKey,
  Connection,
  Transaction,
} from "@solana/web3.js";
import wallet from "./dev-wallet.json"


const from = Keypair.fromSecretKey(new Uint8Array(wallet)) 
const to = new PublicKey("7yH6pP6VhXoPwjzqHQy6hy8tGZUsPfTJzUdKJessUS2M")
const connection = new Connection("https://api.devnet.solana.com","confirmed");


(async ()=>{
    try{

        //get balance
        const balance = await connection.getBalance(from.publicKey);

        //test transaction for fee calculation
        const transaction = new Transaction().add(
            SystemProgram.transfer({
                fromPubkey:from.publicKey,
                toPubkey:to,
                lamports:balance
            })
        )

        transaction.recentBlockhash = (await connection.getRecentBlockhash("confirmed")).blockhash;
        transaction.feePayer = from.publicKey;

        //feee calculation
        const fee = (await connection.getFeeForMessage(transaction.compileMessage())).value || 0;

        //pop the transaction
        transaction.instructions.pop();

        //add new transaction
        transaction.add(
            SystemProgram.transfer({
                fromPubkey:from.publicKey,
                toPubkey:to,
                lamports:balance-fee
            })
        )


        //sign the transaction and confirm it
        const signature = await sendAndConfirmTransaction(
            connection,
            transaction,
            [from]
        )

        console.log(`transfer success !! Check here https://explorer.solana.com/tx/${signature}?cluster=devnet`)
    }catch(e){
        console.log("Got into an error",e)
    }
})();