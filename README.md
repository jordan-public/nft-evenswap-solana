# NFT EvenSwap

I got an NFT on my Saga phone, at the time I was at a crypto meetup in NYC, and someone showed me their NFT (every Saga owner got one a few days ago; Saga Monkes - see: https://magiceden.io/marketplace/sagamonkes?status=%22buy_now%22). Anyway mine was smoking a cigarette and he’s was not, but it had a mohawk. He liked mine better, and me наоборот. So I thought of this simple idea we can implement in a couple of days: NFT Even Swap:

- User A deposits an NFT into our contract. User adds other NFT addresses/ids that he likes. The info is stored in his account that we create for him.
- Other (B) users go in and do the same. As soon as there is a match (and BTW it happens as soon as some user enters a new NFT), we execute the swap: A and B swap their matching IDs and they pay us some Lamports.
- Users can list NFTs that are not (yet) deposited. No problem - as soon as the counterparty realizes that, they can deposit their NFT into our contract and the swap occurs right away.
- Makers pay nothing to list - that’s how we get more listings.
- Takers pay a small price in SOL.
- In the future (maybe same week), we can actually list the candidate tokens for sale at Magiceden or similar for a very high price. This would produce visibility or free advertising for our protocol. And if they sell for a crazy price, even better for the user and for Magiceden which would love us, too. For this the listing into Magiceden has to be performed by our contract and not the user themselves, so the contract can pull it back and swap it if needed.

We can whip this up in a day and learn a lot, what do you think? I hope I’m not throwing a wrench in the gear of our previous discussion.

We can even start swapping NFTs in the same collection. The first version would be calling:
swap(nyNFT, [i like these NFTs])

If any of “i like these NFTs" is in the contract, it will do the swap immediately. If not, it will append them to a list. We can think about efficiency later (like bitmaps for collections etc. - for example Saga MONKES is a limited collection of a few thousand (bits in the bitmap)).

I would even try to write the front end on Solana Mobile if we have time.