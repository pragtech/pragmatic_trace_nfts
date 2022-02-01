use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;
declare_id!("HFYirQPf1g6rHk49HjotXeRV5Nz5wpziQfU4c57Jb9QG");

#[program]
pub mod trace_nfts {

    use super::*;

    pub fn mint_user(
        ctx: Context<MintUser>,
        nft_data: String,
        user_address: Pubkey,
    ) -> ProgramResult {
        // let nft = &mut ctx.accounts.nft;
        let user = &mut ctx.accounts.user;
        let parent = &ctx.accounts.parent;
        if nft_data.chars().count() > 50 {
            return Err(ErrorCode::DATATooLong.into());
        }
        user.owner = user_address;
        user.main_nft.owner = user_address;
        user.main_nft.parent = *parent.key;
        user.nft_count = 1;
        Ok(())
    }

    pub fn mint_nft(ctx: Context<MintNFT>,nft_data: String) -> ProgramResult {
        let nft = &mut ctx.accounts.nft;
        if nft_data.chars().count() > 50 {
            return Err(ErrorCode::DATATooLong.into());
        }
        nft.owner = ctx.accounts.owner.owner;
        nft.data = nft_data;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintUser<'info> {
    // #[account(init,payer = parent,space = 264)]
    // pub nft: Account<'info, NFT>,
    #[account(init,payer = parent,space = 32 + 264)]
    pub user: Account<'info, UserAccount>,
    pub parent: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(init,payer = owner,space = 264)]
    pub nft: Account<'info, NFT>,
    #[account(signer)]
    pub owner: Account<'info,UserAccount>,
    #[account(address = system_program::ID)]
    pub system_program: AccountInfo<'info>,
}

#[account]
pub struct UserAccount {
    pub owner: Pubkey,
    pub main_nft: NFT,
    pub nft_count: i64,
}

#[account]
pub struct NFT {
    pub owner: Pubkey,
    pub parent: Pubkey,
    pub data: String,
}

#[error]
pub enum ErrorCode {
    #[msg("The Provided Nft data is too long")]
    DATATooLong,
}
