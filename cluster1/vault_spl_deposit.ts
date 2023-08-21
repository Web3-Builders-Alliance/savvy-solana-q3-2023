import { Connection, Keypair, SystemProgram, PublicKey, AddressLookupTableAccount, clusterApiUrl, LAMPORTS_PER_SOL } from "@solana/web3.js"
import { Program, Wallet, AnchorProvider, Address, BN } from "@project-serum/anchor"
import { wbaVault, IDL } from "../programs/wba_vault";
import { TOKEN_PROGRAM_ID, getOrCreateAssociatedTokenAccount, mintTo } from '@solana/spl-token';
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

//spl_mint 

const mint = new PublicKey("ECEpMrpoDoeAMBCw4rAvU8iQ21TDWtnmqCcr6KtJQnag");


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
        const ownerAta = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            keypair.publicKey
        )
        const vaultAta = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            vault_auth,
            true
        )
        const txhash = await program.methods
        .depositSpl(
            new BN(2)
        )
        .accounts({
            owner: keypair.publicKey,
            vaultState: vaultState,
            vaultAuth: vault_auth,
            ownerAta: ownerAta.address,
            vaultAta: vaultAta.address,
            tokenMint: mint,
            tokenProgram: TOKEN_PROGRAM_ID       
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