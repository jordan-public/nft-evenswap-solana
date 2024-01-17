
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Transaction, PublicKey, SystemProgram } from "@solana/web3.js";
import { createMint, getOrCreateAssociatedTokenAccount, mintTo, createSetAuthorityInstruction, AuthorityType, getAccount, getMint, Token, TOKEN_PROGRAM_ID, MintLayout } from "@solana/spl-token";
import { Evenswap } from "../target/types/evenswap";

const PROGRAM_ID = new PublicKey("9ABW8EsLa5Muc5a8aKNvi6TDytHDRd9gwx8RqiF1Vfp7");

async function createNFT(connection: anchor.Provider.Connection, creator: anchor.web3.Keypair, recipient: andchor.web3.Keypair) {
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
  const accountInfo = await getAccount(connection, associatedTokenAccount.address);
  // See result
  // console.log("Quantity:", accountInfo.amount);
  // const mintInfo = await getMint(
  //     connection,
  //     mint
  //   );
  // console.log("Mint info:", mintInfo);
  return mint
}

describe("evenswap", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Evenswap as Program<Evenswap>;

  it("Is initialized!", async () => {
    // Call the initialize function here (if necessary, update to reflect your actual function)
    const tx = await program.methods.initialize().rpc();
    console.log("Initialize transaction signature", tx);
  });

  it("Handles swap offers", async () => {
    // Assuming AnchorProvider is set up with a funded wallet
    const provider = anchor.AnchorProvider.env();

    // Generate keypairs for the NFT holder and the swap offerer
    const nftCreator = anchor.web3.Keypair.generate();
    const interestingNftsHolder = anchor.web3.Keypair.generate();
    const swapOfferer = anchor.web3.Keypair.generate();

    // Airdrop 5 sol into each of the above accounts
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(nftCreator.publicKey, 5e9),
      "confirmed"
    );
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(interestingNftsHolder.publicKey, 5e9),
      "confirmed"
    );
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(swapOfferer.publicKey, 5e9),
      "confirmed"
    );
    // Check the balances of the accounts
    //console.log("NFT creator balance", await provider.connection.getBalance(nftCreator.publicKey));
    // console.log("Interesting NFTs holder balance", await provider.connection.getBalance(interestingNftsHolder.publicKey));
    // console.log("Swap offerer balance", await provider.connection.getBalance(swapOfferer.publicKey));
  
    await createNFT(provider.connection, nftCreator, interestingNftsHolder);

    // Mint 3 NFTs for interestingNftsHolder
    const interestingNftMints = await Promise.all([
      createNFT(provider.connection, nftCreator, interestingNftsHolder),
      createNFT(provider.connection, nftCreator, interestingNftsHolder),
      createNFT(provider.connection, nftCreator, interestingNftsHolder),
    ]);
    // Mint 1 NFT for swapOfferer
    const swapOffererNftMint = await createNFT(provider.connection, nftCreator, swapOfferer);

    // Prepare the accounts for the swap_offer function
    const program_token_account = (await getOrCreateAssociatedTokenAccount(provider.connection, nftCreator, swapOffererNftMint, PROGRAM_ID)).address;
    const user_token_account = (await getOrCreateAssociatedTokenAccount(provider.connection, nftCreator, swapOffererNftMint, swapOfferer.publicKey)).address;
    const nft_offer_account = (await PublicKey.findProgramAddress(
      [Buffer.from("nft_offer"), swapOffererNftMint.toBuffer()],
      PROGRAM_ID
    ))[0];

    // Now, call the swap_offer function on your program
    // You need to pass the correct accounts and parameters as required by your program
    // This is just a placeholder and needs to be replaced with actual implementation
    const tx = await program.methods.swapOffer(swapOffererNftMint, interestingNftMints).accounts({
      // List of accounts:
      myNftMint: swapOffererNftMint,
      nftOfferAccount: nft_offer_account,
      userTokenAccount: user_token_account,
      programTokenAccount: program_token_account,
      user: swapOfferer.publicKey,
      programAccount: PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
    }).signers([swapOfferer]).rpc();
    console.log("Swap offer transaction signature", tx);

    // Add assertions here to check the state after the swap offer
    // For example, you might want to check the ownership of the NFTs, the state of the offer, etc.
  });

  it("Handles cancelling offers", async () => {
    // Set up necessary accounts and data for the cancel_offer function
    // Example:
    // const myNftMint = new PublicKey("...");
    // const user = anchor.web3.Keypair.generate();
    // ... (other setup)

    // Call the cancel_offer function
    // const tx = await program.methods.cancelOffer(myNftMint).accounts({...}).rpc();
    // console.log("Cancel offer transaction signature", tx);
  });

  it("Handles swaps", async () => {
    // Set up necessary accounts and data for the swap function
    // Example:
    // const myNftMint = new PublicKey("...");
    // const wantNftMint = new PublicKey("...");
    // const user = anchor.web3.Keypair.generate();
    // ... (other setup)

    // Call the swap function
    // const tx = await program.methods.swap(myNftMint, wantNftMint).accounts({...}).rpc();
    // console.log("Swap transaction signature", tx);
  });

  // Add more tests as needed
});
