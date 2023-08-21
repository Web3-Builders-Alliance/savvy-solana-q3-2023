import { Connection, Keypair, SystemProgram, PublicKey, AddressLookupTableAccount, clusterApiUrl, LAMPORTS_PER_SOL } from "@solana/web3.js"
import { Program, Wallet, AnchorProvider, Address, BN } from "@project-serum/anchor"
import { wbaVault, IDL } from "../programs/wba_vault";
import wallet from "../wba-wallet.json";



// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

// Create a devnet connection
const connection = new Connection(
    clusterApiUrl('devnet'),
    'confirmed'
  );
    

//create vaultstate 
const vaultState = new PublicKey("Ab7HUsrcig6wmHFaDTPvMHkZ2SNji11A7AVkU1mLLbvF");


// Create our anchor provider
const provider = new AnchorProvider(connection, new Wallet(keypair), { commitment: "confirmed"});

// Create our program
const program = new Program<wbaVault>(IDL, "D51uEDHLbWAxNfodfQDv7qkp8WZtxrhi3uganGbNos7o" as Address, provider);

// Create PDA VAULT AUTH
const vault_auth_seeds = [Buffer.from("auth"), vaultState.toBuffer()];
const vault_auth = PublicKey.findProgramAddressSync(vault_auth_seeds, program.programId)[0];

// Create Vault system Program
const vault_seeds = [Buffer.from("vault"), vault_auth.toBuffer()];
const vault = PublicKey.findProgramAddressSync(vault_seeds, program.programId)[0];

// Execute our enrollment transaction
(async () => {
    try {
        const txhash = await program.methods
        .withdraw(
            new BN(0.1*LAMPORTS_PER_SOL)
        )
        .accounts({
            owner: keypair.publicKey,
            vaultState: vaultState,
            vaultAuth: vault_auth,
            vault:vault,
            systemProgram: SystemProgram.programId,
        
        })
        .signers([
            keypair,
            
        ]).rpc();   
        console.log(`Success! Check out your TX here: 
        https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();