import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import wallet from "../wba-wallet.json"
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("ECEpMrpoDoeAMBCw4rAvU8iQ21TDWtnmqCcr6KtJQnag");

// Recipient address
const to = new PublicKey("8htoR8o155Xme1KHFpVHfnHTLGGgWmhxswJRkf9MRwxE");

(async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create it
        const TokenFromWallet = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            keypair.publicKey
        )
        console.log("from", TokenFromWallet.address)

        // Get the token account of the toWallet address, and if it does not exist, create it
        const TokenFromTo = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            to
        )
        console.log("from", TokenFromTo.address)

        // Transfer the new token to the "toTokenAccount" we just created
        const tx = await transfer(
            connection,
            keypair,
            TokenFromWallet.address,
            TokenFromTo.address,
            keypair,
            10000000
        )
        console.log(tx);
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();