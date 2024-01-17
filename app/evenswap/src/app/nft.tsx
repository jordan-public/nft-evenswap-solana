import {WalletNotConnectedError} from '@solana/wallet-adapter-base';
import {useConnection, useWallet} from '@solana/wallet-adapter-react';
import {Keypair, SystemProgram, Transaction} from '@solana/web3.js';
import React, {FC, useCallback} from 'react';
import { createMint, getOrCreateAssociatedTokenAccount, mintTo, createSetAuthorityInstruction, AuthorityType, getAccount, getMint, Token, TOKEN_PROGRAM_ID, MintLayout } from "@solana/spl-token";
import * as anchor from "@coral-xyz/anchor";


const { Connection, clusterApiUrl } = require('@solana/web3.js');


//const connection = new Connection(clusterApiUrl("mainnet-beta"));
const connection = new Connection("http://127.0.0.1:8899", "confirmed");

async function getWalletPublicKey() {
    if (window.solana) {
        try {
            // Connect to the wallet
            await window.solana.connect(); // This will prompt the user to connect their wallet

            // Check if the solana object is connected and has a public key
            if (window.solana.isConnected && window.solana.publicKey) {
                console.log('Connected wallet public key:', window.solana.publicKey.toString());
                return window.solana.publicKey;
            }
        } catch (error) {
            console.error('Error connecting to the wallet:', error);
        }
    } else {
        console.log('Solana object not found! Make sure you have a wallet provider installed.');
    }
    return null;
}

async function createNFT(connection: anchor.Provider.Connection, creator: anchor.web3.Keypair, recipient: anchor.web3.Keypair) {
    // Create a new mint
    const mint = await createMint(
        connection,
        creator,
        creator.publicKey,
        null,
        0
    );
    // Create an account to hold tokens of this new type
    const associatedTokenAccount = await getOrCreateAssociatedTokenAccount(
        connection,
        creator,
        mint,
        recipient.publicKey
    );
    // Mint only one token into the account
    await mintTo(
        connection,
        creator,
        mint,
        associatedTokenAccount.address,
        creator.publicKey,
        1
    );
    // Disable future minting
    let transaction = new Transaction()
        .add(createSetAuthorityInstruction(
            mint,
            creator.publicKey,
            AuthorityType.MintTokens,
            null
        ));
    // const accountInfo = await getAccount(connection, associatedTokenAccount.address);
    // See result
    // const mintInfo = await getMint(
    //     connection,
    //     mint
    //   );
    // console.log("Quantity:", accountInfo.amount);
    // console.log("Mint info:", mintInfo);
    // Return the mint
    return mint
}


export const CreateNFT: FC = () => {
    const {connection} = useConnection();
    const {publicKey, sendTransaction} = useWallet();

    const onClick = useCallback(async () => {
        console.log(publicKey);

        await getWalletPublicKey();



    }, [publicKey, sendTransaction, connection]);

    return (
        <div>
            <button onClick={onClick} disabled={!publicKey}>
                Metaplex connection
            </button>
            <div></div>
            <button onClick={onClick2} disabled={!publicKey}>
                mint
            </button>
        </div>
    );
};