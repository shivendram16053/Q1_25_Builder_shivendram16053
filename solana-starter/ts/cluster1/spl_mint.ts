import { Keypair, PublicKey, Connection, Commitment } from "@solana/web3.js";
import { getOrCreateAssociatedTokenAccount, mintTo } from '@solana/spl-token';
import wallet from "../../../Turbin3_wallet.json";

// Import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

const token_decimals = 1_000_000n;

// Mint address
const mint = new PublicKey("DZQJdB2Kk8JkvfctRRGyC6zeqEWHzzEin5ULXZ7G8h9S");

(async () => {
    try {
        const ata = await getOrCreateAssociatedTokenAccount(connection,keypair,mint,keypair.publicKey);
        console.log(`Associated token account address: ${ata.address.toBase58()}`);
        const signature = await mintTo(connection,keypair,mint,ata.address,keypair.publicKey,1n*token_decimals);
        console.log(`Minted 1 token to the associated token account: ${ata.address.toBase58()}`);
    } catch(error) {
        console.log(`Oops, something went wrong: ${error}`)
    }
})()
