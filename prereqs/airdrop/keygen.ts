import {Keypair} from "@solana/web3.js";

let kp = Keypair.generate();

console.log("created keyair is :",kp.publicKey.toBase58());
console.log([kp.secretKey])