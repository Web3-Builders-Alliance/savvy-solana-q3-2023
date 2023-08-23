import { Metaplex, keypairIdentity, bundlrStorage } from "@metaplex-foundation/js";
import { clusterApiUrl, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from '@solana/web3.js';
import wallet from '../wba-wallet.json';

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

const connection = new Connection(
  clusterApiUrl('devnet'),
  'confirmed'
);

const metaplex = Metaplex.make(connection)
    .use(keypairIdentity(keypair))
    .use(bundlrStorage({
        address: 'https://devnet.bundlr.network',
        providerUrl: "https://api.devnet.solana.com",
        timeout: 60000,
    }));


(async () => {
    try {
        const { nft } = await metaplex.nfts().create(
            {
                uri: "https://arweave.net/Li54wlktUoFgaG8TmcOVjDv2SgAr9XAV9DOZJX2Ky6w",
                name: "generug rug",
                symbol : "BR",
                creators: [{address: keypair.publicKey,
                    share: 100,}],
                sellerFeeBasisPoints: 250,
                isMutable: true
            }
        )
        console.log(nft.address)

    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }


})();