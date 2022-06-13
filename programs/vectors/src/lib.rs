use anchor_lang::prelude::*;

declare_id!("Hdze6SuxHy9nk13FZSrWBiP35m7jfSWKnwkrtYgLYCPL");

#[program]
pub mod vectors {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let vote_account=&mut ctx.accounts.vote_account;
        let my_vector= &mut vote_account.v;
         msg!("Vector initially : {:?}",my_vector);
        my_vector.push(93);
        msg!("Vector after push : {:?}",my_vector);
        my_vector.insert(0, 11);
        msg!("Vector after insert: {:?}",my_vector);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init,payer=user,space=200)]
    pub vote_account: Account<'info, VoteAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    system_program: Program<'info, System>

}

#[account]
pub struct VoteAccount {
    
    v: Vec<u32>
}
