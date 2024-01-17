use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_lang::solana_program::account_info::AccountInfo;
use anchor_spl::token::TokenAccount;
use anchor_spl::token::Transfer;
use anchor_spl::token;

declare_id!("HR1ZNzmJCwUACz69rWDZH9ASRZyEMqXRU3r4ebJzyaRj");

#[program]
pub mod evenswap {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn swap_offer(ctx: Context<SwapOffer>, my_nft_mint: Pubkey, want_nft_mints: Vec<Pubkey>) -> Result<()> {
        // Just because I like to put my_nft_mint in the account data as well as a parameter
        require!(ctx.accounts.my_nft_mint.key() == my_nft_mint, ErrorCode::Unauthorized);
        
        // Ensure that the caller owns the offered NFT account
        require!(ctx.accounts.user_token_account.owner == *ctx.accounts.user.key, ErrorCode::Unauthorized);
        // Also, transfer would fail if the user didn't own the NFT

        // Ensure the provided want_nft_mints vector isn't too large
        require!(want_nft_mints.len() <= 10, ErrorCode::TooManyNfts);

        // Ensure that the offer did not already exist
        // "init" already takes care of this: require!(nft_offer_account.to_account_info().lamports() == 0, ErrorCode::OfferAlreadyExists);

        // Transfer NFT to program
        let cpi_accounts = Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.program_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let program_cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(program_cpi_ctx, 1)?; // Transfer the NFT (quantity 1)

        // Create the new offer account
        // no need as "payer = user" in account macro: **nft_offer_account.to_account_info().try_borrow_mut_lamports()? = Rent::get()?.minimum_balance(nft_offer_account.to_account_info().data_len());
        // no need: nft_offer_account.to_account_info().data.borrow_mut().fill(0); // Zero-initialize the account data

        // Record the offer in the account
        ctx.accounts.nft_offer_account.owner = *ctx.accounts.user.key;
        ctx.accounts.nft_offer_account.my_nft_mint = my_nft_mint;
        ctx.accounts.nft_offer_account.want_nft_mints = want_nft_mints;

        Ok(())
    }

    pub fn cancel_offer(ctx: Context<CancelOffer>, my_nft_mint: Pubkey) -> Result<()> {
        // Just because I like to put my_nft_mint in the account data as well as a parameter
        require!(ctx.accounts.my_nft_mint.key() == my_nft_mint, ErrorCode::Unauthorized);
        
        // Ensure that the nft_offer_account exists
        require!(ctx.accounts.nft_offer_account.to_account_info().lamports() > 0, ErrorCode::AccountAlreadyClosed);

        // Ensure that the owner of the offer is the one cancelling it
        require!(ctx.accounts.nft_offer_account.owner == *ctx.accounts.user.key, ErrorCode::Unauthorized);

        // Transfer would fail if the user didn't own the NFT

        // Transfer NFT to user
        let cpi_accounts = Transfer {
            from: ctx.accounts.program_token_account.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.nft_offer_account.to_account_info(),
        };
        // Create the PDA account's seeds
        let my_nft_mint_key = my_nft_mint.key();
        let seeds = &[
            b"nft_offer",
            my_nft_mint_key.as_ref(),
            &[ctx.bumps.nft_offer_account],
        ];
        let signer = &[&seeds[..]];
        // Complete the transfer
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let program_cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::transfer(program_cpi_ctx, 1)?; // Transfer the NFT (quantity 1)
        
        // Close the account and refund the lamports to the user
        // The Anchor framework handles the transfer as specified by `close = user`
        ctx.accounts.nft_offer_account.close(ctx.accounts.user.to_account_info())?;
        // Zero out the account data
        *ctx.accounts.nft_offer_account.to_account_info().data.borrow_mut() = &mut [];
        
        Ok(())
    }

    pub fn swap(ctx: Context<Swap>, my_nft_mint: Pubkey, want_nft_mint: Pubkey) -> Result<()> {
        // Just because I like to put want_nft_mint in the account data as well as a parameter
        require!(ctx.accounts.want_nft_mint.key() == want_nft_mint, ErrorCode::Unauthorized);
        
        // Ensure that the caller owns the offered NFT account
        require!(ctx.accounts.sent_token_account.owner == *ctx.accounts.user.key, ErrorCode::Unauthorized);

        // Ensure that the program owns the wanted NFT account
        require!(ctx.accounts.program_token_account.owner == ctx.accounts.counterparty_nft_offer_account.key(), ErrorCode::Unauthorized);
        // Ensure that the account corresponds to the wanted NFT
        require!(ctx.accounts.program_token_account.mint == want_nft_mint, ErrorCode::Unauthorized);
        // Also transfer we would fail if the program didn't own the NFT to be received
        
        // Also transfer would fail if the user didn't own the NFT to be sent

        // Ensure that there is an offer for the wanted NFT
        require!(ctx.accounts.counterparty_nft_offer_account.want_nft_mints.contains(&my_nft_mint), ErrorCode::OfferNotFound);

        // Transfer user's NFT to counterparty
        {
            let cpi_accounts = Transfer {
                from: ctx.accounts.sent_token_account.to_account_info(),
                to: ctx.accounts.sent_token_destination_account.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            };
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            token::transfer(cpi_ctx, 1)?; // Assuming NFTs have a quantity of 1
        }

        // Transfer wanted NFT from program to user
        {
            let cpi_accounts = Transfer {
                from: ctx.accounts.program_token_account.to_account_info(),
                to: ctx.accounts.received_token_account.to_account_info(),
                authority: ctx.accounts.counterparty_nft_offer_account.to_account_info(),
            };
            // Create the PDA account's seeds
            let want_nft_mint_key = want_nft_mint.key();
            let seeds = &[
                b"nft_offer",
                want_nft_mint_key.as_ref(),
                &[ctx.bumps.counterparty_nft_offer_account],
            ];
            let signer = &[&seeds[..]];
            // Complete the transfer
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
            token::transfer(cpi_ctx, 1)?; // Assuming NFTs have a quantity of 1
        }

        // Close the account and refund the lamports to the <see next line>
        // Transfer all lamports to the user (this should be the counterparty, not the user, but for simplicity's sake we'll do this for now !!!)
        // The Anchor framework handles the transfer as specified by `close = user`
        ctx.accounts.counterparty_nft_offer_account.close(ctx.accounts.user.to_account_info())?;
        // Zero out the account data
        *ctx.accounts.counterparty_nft_offer_account.to_account_info().data.borrow_mut() = &mut [];

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct SwapOffer<'info> {
    pub my_nft_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = user,
        seeds = [b"nft_offer", my_nft_mint.key().as_ref()],
        bump,
        space = 8 + 32 + 32 + 32 * 10 // Adjust the space as needed
    )]
    pub nft_offer_account: Account<'info, NftOfferAccount>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub program_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: This is to access the program's own account information.
    #[account(executable)]
    pub program_account: AccountInfo<'info>,

    /// CHECK: This is to access the SPL token program. Why unsafe??? !!!
    pub token_program: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CancelOffer<'info> {
    pub my_nft_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"nft_offer", my_nft_mint.key().as_ref()],
        bump,
        close = user,
    )]
    pub nft_offer_account: Account<'info, NftOfferAccount>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub program_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: This is to access the program's own account information.
    #[account(executable)]
    pub program_account: AccountInfo<'info>,

    /// CHECK: This is to access the SPL token program. Why unsafe??? !!!
    pub token_program: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Swap<'info> {
    pub want_nft_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"nft_offer", want_nft_mint.key().as_ref()],
        bump,
        close = user,
    )]
    pub counterparty_nft_offer_account: Account<'info, NftOfferAccount>,

    #[account(mut)]
    pub sent_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub sent_token_destination_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub program_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub received_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: This is to access the program's own account information.
    #[account(executable)]
    pub program_account: AccountInfo<'info>,

    /// CHECK: This is to access the SPL token program. Why unsafe??? !!!
    pub token_program: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct NftOfferAccount {
    pub owner: Pubkey,
    pub my_nft_mint: Pubkey,
    pub want_nft_mints: Vec<Pubkey>,
}

#[error_code]
pub enum ErrorCode {
    // ... (existing error codes)
    #[msg("Too many NFT mint addresses provided")]
    TooManyNfts,
    #[msg("The account has already been closed")]
    AccountAlreadyClosed,
    #[msg("Offer already exists")]
    OfferAlreadyExists,
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Offer not found")]
    OfferNotFound,
}