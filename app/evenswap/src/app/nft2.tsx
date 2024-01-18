import { useWallet } from '@solana/wallet-adapter-react';
import { Connection, PublicKey, SystemProgram, LAMPORTS_PER_SOL, clusterApiUrl} from '@solana/web3.js';
//import { createMint } from '@solana/spl-token';
import * as web3 from '@solana/web3.js';
//import { Token, TOKEN_PROGRAM_ID, MintLayout } from '@solana/spl-token';
import * as splToken from '@solana/spl-token';
import { Metadata, CreateMetadataArgs, Data } from '@metaplex-foundation/mpl-token-metadata';

import { Token, TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID, MintLayout } from '@solana/spl-token';



export function NFTMinter() {
    const wallet = useWallet();

    async function mintNFT3(wallet) {
        try {
            // Check if the wallet is connected
            if (!wallet.connected) {
                console.log('Wallet not connected');
                return;
            }

            // Establish a connection to the Solana blockchain
            const connection = new Connection("http://127.0.0.1:8899", "confirmed");

            // Create a new account to hold the minted NFT
            const mintAccount = await PublicKey.createWithSeed(
                wallet.publicKey,
                'nft',
                TOKEN_PROGRAM_ID
            );

            // Get the rent-exempt balance for the mint account
            const rentExemptBalance = await connection.getMinimumBalanceForRentExemption(MintLayout.span);

            // Create a transaction to mint the NFT
            let transaction = new Token(
                connection,
                mintAccount,
                TOKEN_PROGRAM_ID,
                wallet.publicKey // Wallet public key as the payer
            ).createMintToInstruction(
                wallet.publicKey, // Mint authority
                mintAccount,      // Mint to this account
                1,                // Amount to mint (1 for an NFT)
                0                 // Decimals (0 for an NFT)
            );

            // Sign and send the transaction
            let signature = await wallet.sendTransaction(transaction, connection);
            await connection.confirmTransaction(signature, 'confirmed');

            console.log('NFT minted: ', signature);
        } catch (error) {
            console.error('Error minting NFT:', error);
        }
    }

    async function createNFT3() {

        mintNFT3(wallet);
    }

    async function mintNFT(wallet, connection, metadataUri) {
        // Create a new mint account
        const mint = web3.Keypair.generate();
        const mintRent = await connection.getMinimumBalanceForRentExemption(splToken.MintLayout.span);
        const transaction = new web3.Transaction();

        // Create account transaction
        transaction.add(web3.SystemProgram.createAccount({
            fromPubkey: wallet.publicKey,
            newAccountPubkey: mint.publicKey,
            space: splToken.MintLayout.span,
            lamports: mintRent,
            programId: splToken.TOKEN_PROGRAM_ID,
        }));

        // Initialize mint transaction
        transaction.add(splToken.Token.createInitMintInstruction(
            splToken.TOKEN_PROGRAM_ID,
            mint.publicKey,
            0,
            wallet.publicKey,
            wallet.publicKey
        ));

        // Create the associated token account for the wallet
        const tokenAccount = await splToken.Token.getAssociatedTokenAddress(
            splToken.ASSOCIATED_TOKEN_PROGRAM_ID,
            splToken.TOKEN_PROGRAM_ID,
            mint.publicKey,
            wallet.publicKey
        );
        transaction.add(splToken.Token.createAssociatedTokenAccountInstruction(
            splToken.ASSOCIATED_TOKEN_PROGRAM_ID,
            splToken.TOKEN_PROGRAM_ID,
            mint.publicKey,
            tokenAccount,
            wallet.publicKey,
            wallet.publicKey
        ));

        // Mint to transaction
        transaction.add(splToken.Token.createMintToInstruction(
            splToken.TOKEN_PROGRAM_ID,
            mint.publicKey,
            tokenAccount,
            wallet.publicKey,
            [],
            1 // Mint one token
        ));

        // Create metadata transaction
        const metadataData = new Data({
            symbol: "NFTSYM",
            name: "My NFT",
            uri: metadataUri,
            sellerFeeBasisPoints: 500, // Represents a 5.00% seller fee.
            creators: null
        });
        const createMetadataArgs = new CreateMetadataArgs({ data: metadataData, isMutable: true });
        const metadataPDA = await Metadata.getPDA(mint.publicKey);
        transaction.add(new Metadata(
            'CreateMetadata',
            {
                metadata: metadataPDA,
                mint: mint.publicKey,
                mintAuthority: wallet.publicKey,
                payer: wallet.publicKey,
                updateAuthority: wallet.publicKey,
            },
            createMetadataArgs
        ).toInstruction());

        // Sign and send transaction
        transaction.recentBlockhash = (await connection.getRecentBlockhash()).blockhash;
        transaction.feePayer = wallet.publicKey;
        await wallet.signTransaction(transaction);
        const rawTransaction = transaction.serialize();
        const txId = await connection.sendRawTransaction(rawTransaction);

        await connection.confirmTransaction(txId);
        return mint.publicKey;
    }

    async function createNFT() {
        const connection = new Connection("http://127.0.0.1:8899", "confirmed");
        mintNFT(wallet, connection, 'https://arweave.net/your-metadata-uri');
    }

    async function createNewMint() {
        if (!wallet.connected) {
            console.log("Wallet is not connected");
            return;
        }

        const connection = new Connection(clusterApiUrl('devnet'), 'confirmed');
        //const connection = new Connection("http://127.0.0.1:8899", "confirmed");

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

    const mintSPLToken = async (wallet) => {
        if (!wallet.publicKey) throw new Error('Wallet is not connected');

        const connection = new Connection(clusterApiUrl('devnet'), 'confirmed');

        // Create a new token mint
        debugger;
        const mint = await Token.createMint(
            connection,
            wallet, // payer
            wallet.publicKey, // mint authority
            null, // freeze authority (null if not needed)
            0, // decimals
            TOKEN_PROGRAM_ID
        );

        // Create the associated token account for the wallet
        const tokenAccount = await mint.getOrCreateAssociatedAccountInfo(
            wallet.publicKey
        );

        // Mint 1 new token (NFT) to the token account
        await mint.mintTo(
            tokenAccount.address,
            wallet.publicKey, // authority to mint
            [], // multisig
            1 // amount, set to 1 for NFT
        );

        console.log(`Minted 1 SPL Token: ${mint.publicKey.toBase58()}`);
        console.log(`Token Account: ${tokenAccount.address.toBase58()}`);
    };

    async function createNFT4() {
        mintSPLToken(wallet);
    }


    return (
        <div>
            <button onClick={createNewMint}>Create New Mint</button>
            <div></div>
            <button onClick={createNFT}>Create NFT</button>
            <div></div>
            <button onClick={createNFT4}>Create NFT4</button>
        </div>
    );
}
