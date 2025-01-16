import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import wallet from "../../Turbin3_wallet.json"
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("DZQJdB2Kk8JkvfctRRGyC6zeqEWHzzEin5ULXZ7G8h9S");

// Recipient address
const to = new PublicKey("AsAduBWNpjJXvW2mN1PXKM1CuHeNYapQQ2VCjZpq9Hbq");

(async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create it
        const fromata = await getOrCreateAssociatedTokenAccount(connection,keypair,mint,keypair.publicKey);

        // Get the token account of the toWallet address, and if it does not exist, create it
        const toata = await getOrCreateAssociatedTokenAccount(connection,keypair,mint,to);

        // Transfer the new token to the "toTokenAccount" we just created
        const tx = await transfer(connection,keypair,fromata.address,toata.address,keypair,1e6);
        console.log("transaction done",tx);
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();