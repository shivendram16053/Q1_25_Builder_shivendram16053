import { Connection, Keypair, PublicKey } from "@solana/web3.js"
import { Program, Wallet, AnchorProvider } from "@coral-xyz/anchor"
import { IDL, Turbin3Prereq } from "./programs/Turbin3_prereq";
import { Idl } from "@coral-xyz/anchor";
import wallet from "./Turbin3_wallet.json"

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

const connection = new Connection("https://api.devnet.solana.com","confirmed");

const github = Buffer.from("shivendram16053","utf-8")

const provider = new AnchorProvider(connection,new Wallet(keypair),{commitment:"confirmed"});

const program :Program<Idl>=new Program(IDL as Idl,provider);

const enrollment_seeds = [Buffer.from("prereq"),keypair.publicKey.toBuffer()];

const [enrollment_key,_bump] = PublicKey.findProgramAddressSync(enrollment_seeds,program.programId);

(
    async ()=>{
        try{
            const txHash = await program.methods.complete(github).accounts({signer:keypair.publicKey}).signers([keypair]).rpc();
            console.log(`Success !! check here https://explorer.solana.com/tx/${txHash}?cluster=devnet`);
            
        }catch(e){
            console.log("Got an error",e)
        }
    }
)();