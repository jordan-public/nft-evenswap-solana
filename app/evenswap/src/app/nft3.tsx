import { useWallet } from '@solana/wallet-adapter-react';
import { Connection, PublicKey, clusterApiUrl, Keypair, LAMPORTS_PER_SOL, SystemProgram, Transaction } from '@solana/web3.js';
import { createMint, Token, TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID } from '@solana/spl-token';

import * as web3 from '@solana/web3.js';

export function NFTMinter3() {
    const wallet = useWallet();

    async function mintSPLToken(wallet) {
        if (!wallet.connected) {
            // Connect to the wallet
            await wallet.connect();
        }

        const connection = new Connection('https://api.devnet.solana.com', 'confirmed');
        debugger;

        const mintA = web3.Keypair.generate();

        // Create a new mint
        const mint = await createMint(
            connection,
            wallet, // payer
            wallet.publicKey, // mint authority
            null, // freeze authority (null if not needed)
            0, // decimals
            undefined,
            {},
            TOKEN_PROGRAM_ID
        );


        // Create the associated token account for this wallet
        let tokenAccount = await mint.getOrCreateAssociatedAccountInfo(wallet.publicKey);

        // Mint 1 new token (NFT) to the token account
        let signature = await mint.mintTo(
            tokenAccount.address, // account to mint to
            wallet.publicKey, // minting authority
            [], // multisig
            1 // amount to mint
        );

        console.log("Minted 1 SPL Token:", mint.publicKey.toString());
        console.log("Transaction Signature:", signature);
    }
    async function createNFT() {
        //const connection = new Connection(clusterApiUrl('devnet'), 'confirmed');

        mintSPLToken(wallet);
    }


    return (
        <div>

            <button onClick={createNFT}>Create NFT</button>
        </div>
    );
}
