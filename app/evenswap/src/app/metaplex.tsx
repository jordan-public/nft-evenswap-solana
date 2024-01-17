import {WalletNotConnectedError} from '@solana/wallet-adapter-base';
import {useConnection, useWallet} from '@solana/wallet-adapter-react';
import {Keypair, SystemProgram, Transaction} from '@solana/web3.js';
import React, {FC, useCallback} from 'react';

import {Metaplex} from "@metaplex-foundation/js";
import {Connection, clusterApiUrl} from "@solana/web3.js";

//const connection = new Connection(clusterApiUrl("mainnet-beta"));
const connection = new Connection("http://127.0.0.1:8899", "confirmed");
const metaplex = new Metaplex(connection);

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

export const MetaplexConnection: FC = () => {
    const {connection} = useConnection();
    const {publicKey, sendTransaction} = useWallet();

    const onClick = useCallback(async () => {
        console.log(publicKey);

        await getWalletPublicKey();

        const myNfts = await metaplex.nfts().findAllByOwner({
            owner: publicKey
        });
        console.log(myNfts);

    }, [publicKey, sendTransaction, connection]);

    const onClick2 = useCallback(async () => {
        console.log("mint nft");

        const {nft} = await metaplex
            .nfts()
            .create({
                uri: "https://arweave.net/123",
                name: "My NFT",
                sellerFeeBasisPoints: 500 // Represents 5.00%.
            })
            //.run();

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