import { useWallet } from '@solana/wallet-adapter-react';
import { Connection, PublicKey, SystemProgram, LAMPORTS_PER_SOL } from '@solana/web3.js';
import { createMint } from '@solana/spl-token';
import * as web3 from '@solana/web3.js';
//import { Token, TOKEN_PROGRAM_ID, MintLayout } from '@solana/spl-token';
import * as splToken from '@solana/spl-token';


export function NFTMinter() {
    const wallet = useWallet();

    async function createNewMint() {
        if (!wallet.connected) {
            console.log("Wallet is not connected");
            return;
        }

        //const connection = new Connection(clusterApiUrl('devnet'), 'confirmed');
        const connection = new Connection("http://127.0.0.1:8899", "confirmed");

        // Create a new keypair for the mint
        const mint = web3.Keypair.generate();

        // Determine the amount of SOL needed to pay for the rent exemption
        const lamports = await connection.getMinimumBalanceForRentExemption(
            splToken.MintLayout.span
        );

        const transaction = new web3.Transaction().add(
            SystemProgram.createAccount({
                fromPubkey: wallet.publicKey,
                newAccountPubkey: mint.publicKey,
                space: splToken.MintLayout.span,
                lamports,
                programId: splToken.TOKEN_PROGRAM_ID,
            }),
            splToken.createInitializeMintInstruction(
                mint.publicKey, // mint pubkey
                0, // decimals
                wallet.publicKey, // mint authority
                null, // freeze authority (optional)
                splToken.TOKEN_PROGRAM_ID // token program id
            )
        );

        // Send the transaction
        //debugger;
        //const signature = await wallet.sendTransaction(transaction, connection);
        // const signature = await web3.sendAndConfirmTransaction(
        //     connection,
        //     transaction,
        //     [mint]
        // );

        // Sign and send the transaction
        try {
            const signature = await wallet.sendTransaction(transaction, connection);
            await connection.confirmTransaction(signature, 'confirmed');

            console.log('Transaction confirmed with signature:', signature);
        } catch (err) {
            console.log("error", err);
            //debugger;
        }


    }

    return (
        <div>
            <button onClick={createNewMint}>Create New Mint</button>
    </div>
);
}
