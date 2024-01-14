use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
//use anchor_lang::solana_program::program_pack::Pack;
use anchor_lang::solana_program::account_info::AccountInfo;
//use anchor_lang::AccountsExit;
use anchor_spl::token::TokenAccount;
use anchor_spl::token::Transfer;
use anchor_spl::token;

declare_id!("8KswSMtEQ9sjHkZ7ogYy1e2wcnntFnCJigNijTFiQEfe");

#[program]
pub mod evenswap {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn swap_offer(ctx: Context<SwapOffer>, my_nft_mint: Pubkey, want_nft_mints: Vec<Pubkey>) -> Result<()> {
        let user = &ctx.accounts.user;
        let user_token_account_info = &ctx.accounts.user_token_account;
        let program_token_account_info = &ctx.accounts.program_token_account;
        let nft_offer_account = &mut ctx.accounts.nft_offer_account;

        // Just because I like to put my_nft_mint in the account data as well as a parameter
        require!(ctx.accounts.my_nft_mint.key() == my_nft_mint, ErrorCode::Unauthorized);
        
        // Ensure that the caller owns the offered NFT
        require!(user_token_account_info.owner == *user.key, ErrorCode::Unauthorized);

        // Ensure the provided want_nft_mints vector isn't too large
        require!(want_nft_mints.len() <= 10, ErrorCode::TooManyNfts);

        // Ensure that the offer did not already exist
        require!(nft_offer_account.to_account_info().lamports() == 0, ErrorCode::OfferAlreadyExists);

        // Transfer NFT to program
        let cpi_accounts = Transfer {
            from: user_token_account_info.to_account_info(),
            to: program_token_account_info.to_account_info(),
            authority: user.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let program_cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(program_cpi_ctx, 1)?; // Transfer the NFT (quantity 1)

        // Create the new offer account
        // no need as "payer = user" in account macro: **nft_offer_account.to_account_info().try_borrow_mut_lamports()? = Rent::get()?.minimum_balance(nft_offer_account.to_account_info().data_len());
        // no need: nft_offer_account.to_account_info().data.borrow_mut().fill(0); // Zero-initialize the account data

        // Record the offer in the account
        nft_offer_account.owner = *user.key;
        nft_offer_account.my_nft_mint = my_nft_mint;
        nft_offer_account.want_nft_mints = want_nft_mints;

        Ok(())
    }

    pub fn cancel_offer(ctx: Context<CancelOffer>, my_nft_mint: Pubkey) -> Result<()> {
        // Close the PDA account
        let nft_offer_account = &ctx.accounts.nft_offer_account;
        let user = &ctx.accounts.user;

        // Just because I like to put my_nft_mint in the account data as well as a parameter
        require!(ctx.accounts.my_nft_mint.key() == my_nft_mint, ErrorCode::Unauthorized);
        
        // Ensure that the nft_offer_account exists
        require!(nft_offer_account.to_account_info().lamports() > 0, ErrorCode::AccountAlreadyClosed);

        // Ensure that the owner of the offer is the one cancelling it
        require!(nft_offer_account.owner == *user.key, ErrorCode::Unauthorized);

        // Close the account and refund the lamports to the user
        // The Anchor framework handles the transfer as specified by `close = user`
        ctx.accounts.nft_offer_account.close(ctx.accounts.user.to_account_info())?;
        // Zero out the account data
        *nft_offer_account.to_account_info().data.borrow_mut() = &mut [];
        
        // Transfer NFT to user
        let cpi_accounts = Transfer {
            from: ctx.accounts.program_token_account.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.program_account.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let program_cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(program_cpi_ctx, 1)?; // Transfer the NFT (quantity 1)
        
        Ok(())
    }

    pub fn swap(ctx: Context<Swap>, my_nft_mint: Pubkey, want_nft_mint: Pubkey) -> Result<()> {
        let user = &ctx.accounts.user;
        let user_token_account_info = &ctx.accounts.user_token_account;
        let program_token_account_info = &ctx.accounts.program_token_account;
        let counterparty_token_account_info = &ctx.accounts.counterparty_token_account;
        let counterparty_nft_offer_account_info = &ctx.accounts.counterparty_nft_offer_account;
        
        // Just because I like to put want_nft_mint in the account data as well as a parameter
        require!(ctx.accounts.want_nft_mint.key() == want_nft_mint, ErrorCode::Unauthorized);
        
        // Ensure that the caller owns the offered NFT
        require!(user_token_account_info.owner == *user.key, ErrorCode::Unauthorized);

        // Ensure that the program owns the wanted NFT
        require!(program_token_account_info.owner == *ctx.accounts.program_account.key, ErrorCode::Unauthorized);
        require!(program_token_account_info.mint == want_nft_mint, ErrorCode::Unauthorized);

        // Ensure that there is an offer for the wanted NFT
        require!(counterparty_nft_offer_account_info.want_nft_mints.contains(&my_nft_mint), ErrorCode::OfferNotFound);

        // Close the account and refund the lamports to the <see next line>
        // Transfer all lamports to the user (this should be the counterparty, not the user, but for simplicity's sake we'll do this for now !!!)
        // The Anchor framework handles the transfer as specified by `close = user`
        ctx.accounts.counterparty_nft_offer_account.close(ctx.accounts.user.to_account_info())?;
        // Zero out the account data
        *counterparty_nft_offer_account_info.to_account_info().data.borrow_mut() = &mut [];

        // Transfer user's NFT to counterparty
        {
            let cpi_accounts = Transfer {
                from: user_token_account_info.to_account_info(),
                to: counterparty_token_account_info.to_account_info(),
                authority: user.to_account_info(),
            };
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            token::transfer(cpi_ctx, 1)?; // Assuming NFTs have a quantity of 1
        }

        // Transfer wanted NFT from program to user
        {
            let cpi_accounts = Transfer {
                from: program_token_account_info.to_account_info(),
                to: user_token_account_info.to_account_info(),
                authority: ctx.accounts.program_account.to_account_info(),
            };
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            token::transfer(cpi_ctx, 1)?; // Assuming NFTs have a quantity of 1
        }

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
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub program_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub counterparty_token_account: Account<'info, TokenAccount>,

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