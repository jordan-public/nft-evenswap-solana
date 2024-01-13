use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
//use anchor_lang::solana_program::program_pack::Pack;
use anchor_lang::solana_program::account_info::AccountInfo;
//use anchor_lang::AccountsExit;
use anchor_spl::token::TokenAccount;

declare_id!("8KswSMtEQ9sjHkZ7ogYy1e2wcnntFnCJigNijTFiQEfe");

#[program]
pub mod evenswap {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn swap(ctx: Context<Swap>, my_nft_mint: Pubkey, want_nft_mints: Vec<Pubkey>) -> Result<()> {
        let user = &ctx.accounts.user;
        let program_account_info = &ctx.accounts.program_account;
        let mut match_found = false;
        let nft_offer_account = &ctx.accounts.nft_offer_account;
        
        // Ensure that the caller owns the offered NFT
        require!(nft_offer_account.owner == *user.key, ErrorCode::Unauthorized);

        // Ensure the provided want_nft_mints vector isn't too large
        require!(want_nft_mints.len() <= 10, ErrorCode::TooManyNfts);

        for want_nft_mint in want_nft_mints {
            // Load and validate the counterparty offer account
            if let counterparty_offer_pda = find_offer_pda(&want_nft_mint, &ctx.program_id) {
                let counterparty_offer = Account::<NftOfferAccount>::try_from(&counterparty_offer_pda)?;
                if counterparty_offer.want_nft_mints.contains(&my_nft_mint) {
                    // Perform the swap
                    perform_swap(&ctx, &counterparty_offer, &user, want_nft_mint, &my_nft_mint)?;
                    
                    // Close the counterparty account
                    close_offer(counterparty_offer_pda, counterparty_offer.owner)?;
    
                    match_found = true;
                    break;
                }
            }
        }
    
        if !match_found {
            // Transfer user's NFT to counterparty
            let cpi_accounts = Transfer {
                from: user_token_account.to_account_info(),
                to: program_token_account.to_account_info(),
                authority: user.to_account_info(),
            };
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            token::transfer(cpi_ctx, 1)?; // Assuming NFTs have a quantity of 1

            create_waiting_offer_account(ctx, user, my_nft_mint, want_nft_mints)?;
        }
    
        Ok(())
    }
    
    // Helper function to find the offer account
    fn find_offer_pda(
        offered_nft_mint: &Pubkey,
        program_id: &Pubkey
    ) -> Pubkey {
        // Derive the PDA
        let seeds = &[b"nft_offer", offered_nft_mint];
        let (pda, _bump) = Pubkey::find_program_address(seeds, program_id);
        pda
    }
    
    fn perform_swap(
        ctx: &Context<SwapContext>, // You need to define SwapContext with necessary accounts
        counterparty_offer_account: &Account<NftOfferAccount>,
        user: &Signer,
        want_nft_mint: &Pubkey,
        my_nft_mint: &Pubkey
    ) -> Result<(), ErrorCode> {
        // Assume user_token_account and counterparty_token_account are part of SwapContext
        let user_token_account = &ctx.accounts.user_token_account;
        let counterparty_token_account = &ctx.accounts.counterparty_token_account;
        let program_token_account = &ctx.accounts.program_token_account;
    
        // Transfer user's NFT to counterparty
        let cpi_accounts = Transfer {
            from: user_token_account.to_account_info(),
            to: counterparty_token_account.to_account_info(),
            authority: user.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, 1)?; // Assuming NFTs have a quantity of 1
    
        // Transfer counterparty's NFT to user
        // Assuming that the NFT is already held by the smart contract
        let program_to_user_transfer = Transfer {
            from: ctx.accounts.program_token_account.to_account_info(), // NFT account held by the program
            to: user_token_account.to_account_info(), // User's NFT account
            authority: ctx.accounts.program_account.to_account_info(), // Program's authority
        };
        let program_cpi_ctx = CpiContext::new(cpi_program.clone(), program_to_user_transfer);
        token::transfer(program_cpi_ctx, 1)?; // Transfer the NFT (quantity 1)

        Ok(())
    }
    
    // Helper function to close the counterparty account
    fn close_offer(offer_account_pda: Pubkey, recipient: &AccountInfo) -> Result<(), ProgramError> {
        // Transfer all lamports to the recipient
        **recipient.try_borrow_mut_lamports()? += **offer_account.to_account_info().lamports.borrow();
        **offer_account.to_account_info().try_borrow_mut_lamports()? = 0;
    
        // Zero out the account data
        *offer_account.to_account_info().data.borrow_mut() = &mut [];
    
        Ok(())
    }
    
    // Helper function to create a new waiting offer account
    fn create_waiting_offer_account(ctx: Context<Swap>, user: &Signer, my_nft_mint: Pubkey, want_nft_mints: Vec<Pubkey>) -> Result<(), ErrorCode> {
        // Ensure the list of wanted NFTs is not too large
        require!(want_nft_mints.len() <= 10, ErrorCode::TooManyNfts);

        // Derive the PDA for the new offer account
        let pda = find_offer_pda(my_nft_mint.as_ref(), &ctx.program_id);

        // Initialize the new offer account
        let nft_offer_account = &mut ctx.accounts.nft_offer_account;
        **nft_offer_account.to_account_info().try_borrow_mut_lamports()? = Rent::get()?.minimum_balance(nft_offer_account.to_account_info().data_len());
        nft_offer_account.to_account_info().data.borrow_mut().fill(0); // Zero-initialize the account data

        // Set the account data
        nft_offer_account.owner = *user.key;
        nft_offer_account.want_nft_mints = want_nft_mints;

        Ok(())
    }

    pub fn cancel(ctx: Context<Cancel>, my_nft_mint: Pubkey) -> Result<()> {
        // Close the PDA account
        let nft_offer_account = &ctx.accounts.nft_offer_account;
        let owner = &ctx.accounts.user;

        // Ensure that the account has not already been closed
        require!(nft_offer_account.to_account_info().lamports() > 0, ErrorCode::AccountAlreadyClosed);

        // Ensure that the owner of the offer is the one cancelling it
        require!(nft_offer_account.owner == *owner.key, ErrorCode::Unauthorized);

        // Close the account and refund the lamports to the owner
        close_offer(nft_offer_account, owner.to_account_info());
        // Or maybe !!!
        // let pda = find_offer_pda(my_nft_mint.as_ref(), &ctx.program_id);
        // close_offer(pda, owner.to_account_info());

        // Transfer NFT to user
        // Assuming that the NFT is already held by the smart contract
        let program_to_user_transfer = Transfer {
            from: ctx.accounts.program_token_account.to_account_info(), // NFT account held by the program
            to: user_token_account.to_account_info(), // User's NFT account
            authority: ctx.accounts.program_account.to_account_info(), // Program's authority
        };
        let program_cpi_ctx = CpiContext::new(cpi_program.clone(), program_to_user_transfer);
        token::transfer(program_cpi_ctx, 1)?; // Transfer the NFT (quantity 1)
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(
        init,
        payer = user,
        seeds = [b"nft_offer", my_nft_mint.key().as_ref()],
        bump,
        space = 8 + 32 + 32 * 10 // Adjust the space as needed
    )]
    pub nft_offer_account: Account<'info, NftOfferAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub user_token_account: AccountInfo<'info>,

    #[account(mut)]
    pub program_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub counterparty_token_account: Account<'info, TokenAccount>,

    pub my_nft_mint: Account<'info, Mint>,

    /// CHECK: This is to access the program's own account information.
    #[account(executable)]
    pub program_account: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Cancel<'info> {
    #[account(
        mut,
        seeds = [b"nft_offer", my_nft_mint.key().as_ref()],
        bump,
        close = user
    )]
    pub nft_offer_account: Account<'info, NftOfferAccount>,

    #[account(mut)]
    pub user_token_account: AccountInfo<'info>,

    #[account(mut)]
    pub program_token_account: Account<'info, TokenAccount>,

    pub my_nft_mint: Account<'info, Mint>,

    #[account(mut)]
    pub user: Signer<'info>,
}

#[account]
pub struct NftOfferAccount {
    pub owner: Pubkey,
    pub want_nft_mints: Vec<Pubkey>,
}

#[error_code]
pub enum ErrorCode {
    // ... (existing error codes)
    #[msg("Too many NFT mint addresses provided")]
    TooManyNfts,
    #[msg("The account has already been closed")]
    AccountAlreadyClosed,
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Counterparty offer account not found")]
    CounterpartyAccountNotFound,
}