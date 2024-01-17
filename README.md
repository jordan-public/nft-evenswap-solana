# NFT EvenSwap

## Development environment setup

- Install Visual Studio Code
- Install Gitpod extension in the above
- In the browser go to [the Gitpod instance](https://gitpod.io/#github.com/jordan-public/nft-evenswap-solana) to start/open it. It will run very slowly the first time, but then next time around it will be ok.

---

##  Build and deploy

To build the project, at the project root run:
```shell
anchor build
```
---

To test the project, at the project root run:
```shell
anchor test
```

To test the project against the separate Solana Test Validator or any other Solana Validator, configure the validator connection in ```Anchor.toml``` and then run the test as follows:
```shell
anchor test --skip-local-validator
```

For example, to run the test against the locally started Solana Test Validator, in one shell tun the validator:
```shell
solana-test-validator
```

Then without any changes to ```Anchor.toml``` in another shell, from the root of the project, run:
```shell
anchor test --skip-local-validator
```

---
To deploy the project to the local test node, first run the solana test node:

Then, at the project root run:
```shell
solana-test-validator
```

```shell
solana program deploy target/deploy/evenswap.so
```

Set the Program Id in 3 files: lib.rs, Anchor.toml, evenswap.ts

To deploy it to devnet, at the project root run:
```shell
solana program deploy target/deploy/evenswap.so -u devnet
```
---

## Demo

solana airdrop <AMOUNT> <YOUR_WALLET_ADDRESS> --url http://127.0.0.1:8899
solana airdrop 5 DTLJmZn4SEpx8xRYcqCTxNEEd6WyjLfctBJuD9qQbjGe --url http://127.0.0.1:8899

TBD

---

## Introduction

I got an NFT on my Saga phone, at the time I was at a crypto meetup in NYC, and someone showed me their NFT (every Saga owner got one a few days ago; Saga Monkes - see: https://magiceden.io/marketplace/sagamonkes?status=%22buy_now%22). Anyway mine was smoking a cigarette and he’s was not, but it had a mohawk. He liked mine better, I liked his. So I thought of this simple idea we can implement in a couple of days: NFT Even Swap.

## Protocol

Any user interested in swapping his NFT for any NFT in his list of desired NFTs can call:
```
swap_offer(my_nft_mint: Pubkey, i_want_nfts: Vec<Pubkey>)
```
This call records the offer on-chain, so others can take the offer.
The NFT with mint ```my_nft_mint``` is deposited into the program's appropriate Token account.
The caller does not pay for this action, other than the Lamports for the network fees.
This call will fail in the following cases:
- The caller does not own the NFT with mint ```my_nft_mint```,
- The list of desired NFTs with Pubkey-s ```i_want_nfts``` is longer than 10 (this is a consrtaint in the program, and can be changed in the future),
---
Any user can cancel an offer by calling:
```
cancel_offer(my_nft_mint)
```
This call removes the offer to swap the NFT with mint ```my_nft_mint``` previously recorded on-chain.
The NFT with mint ```my_nft_mint``` is returned to the caller's appropriate Token account.
The caller does not pay for this action, other than the Lamports for the network fees.
This call will fail in the following cases:
- The caller does not own the NFT with mint ```my_nft_mint```,
- There is no previous offer recorded for swapping the NFT with mint Pubkey ```my_nft_mint```.
---
Any user can execute a swap by calling:
```
swap(my_nft_mint: Pubkey, other_nft_mint: Pubkey)
```
If a previously created offer exists for this NFT, then this call swaps the NFTs as follows:
- The NFT with mint ```my_nft_mint``` is transferred from the caller's appropriate Token account to the appropriate Token account of the caller who placed the existing offer of his NFT with mint ```other_nft_mint```.
- The NFT with mint ```other_nft_mint``` is transferred from the program's appropriate Token account to the appropriate Token account of the caller.

The caller pays a Protocol Fee for this action, in addition to the Lamports for the network fees.
This call will fail in the following cases:
- The caller does not own the NFT with mint ```my_nft_mint```,
- There is a previous offer recorded for swapping the NFT with mint Pubkey ```my_nft_mint```.
- There is no previous offer recorded for swapping the NFT with mint Pubkey ```other_nft_mint```.

## Protocol Fees (future work)
- Makers pay nothing to list - that’s how we get more listings.
- Takers pay a small price in SOL.

## Listing (future work)

The offers for swapping are listed and displayed in the front end via discovery using [this RPC call](https://solanacookbook.com/guides/get-program-accounts.html#facts).

## Free Advertising (future work)
In the future (maybe same week), we can actually list the candidate tokens for sale at Magiceden or similar for a very high price. This would produce visibility or free advertising for our protocol. And if they sell for a crazy price, even better for the user and for Magiceden which would love us, too. For this the listing into Magiceden has to be performed by our contract and not the user themselves, so the contract can pull it back and swap it if needed.

## Solana Mobile Front End (future work)

Using React Native and its Solana Mobile integration.