import { Connection, Keypair, SystemProgram, PublicKey, clusterApiUrl } from "@solana/web3.js"
import { Program, Wallet, AnchorProvider, Address } from "@project-serum/anchor"
import { wbaVault, IDL } from "../programs/wba_vault";
import wallet from "../wba-wallet.json"


// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

// Create a devnet connection
const connection = new Connection(
    clusterApiUrl('devnet'),
    'confirmed'
  );

//create vaultstate 

const vaultState =Keypair.generate();
console.log(`PK: ${vaultState.secretKey}`);
console.log(`PB: ${vaultState.publicKey}`);

const provider = new AnchorProvider(connection, new Wallet(keypair), { commitment: "confirmed"});

// Create our program
const program = new Program<wbaVault>(IDL, "D51uEDHLbWAxNfodfQDv7qkp8WZtxrhi3uganGbNos7o" as Address, provider);

// Create PDA VAULT AUTH
const vault_auth_seeds = [Buffer.from("auth"), vaultState.publicKey.toBuffer()];
const vault_auth = PublicKey.findProgramAddressSync(vault_auth_seeds, program.programId)[0];

// Create Vault system Program
const vault_seeds = [Buffer.from("vault"), vault_auth.toBuffer()];
const vault = PublicKey.findProgramAddressSync(vault_seeds, program.programId)[0];

// Execute our enrollment transaction
(async () => {
    try {
        const txhash = await program.methods
        .initialize()
        .accounts({
            owner: keypair.publicKey,
            vaultState: vaultState.publicKey,
            vaultAuth: vault_auth,
            vault:vault,
            systemProgram: SystemProgram.programId,
        
        })
        .signers([
            keypair,
            vaultState

        ]).rpc();   
        console.log(`Success! Check out your TX here: 
        https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();