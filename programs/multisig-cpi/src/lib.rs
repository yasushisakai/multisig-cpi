use anchor_lang::prelude::*;

declare_id!("9VtuF6VxDRjUMD1XVfLooYbdkuYwyLA8Us7P1uhYYutq");

#[program]
pub mod multisig_cpi {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, authority: Pubkey) -> Result<()> {

        let data = &mut ctx.accounts.data;
        data.authority = authority;

        Ok(())
    }

    pub fn update_value(ctx: Context<UpdateData>, value: u8) -> Result<()> {

        let data = &mut ctx.accounts.data;
        data.value = value;
        
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(authority:Pubkey)]
pub struct Initialize<'info> {
    #[account(init, payer=payer, space=Data::SIZE)]
    data: Account<'info, Data>,
    #[account(mut)]
    payer: Signer<'info>,
    system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct UpdateData<'info>{
    #[account(mut,has_one=authority)]
    data: Account<'info, Data>,
    authority: Signer<'info>
}

#[account]
pub struct Data {
   value: u8,
   authority: Pubkey,
}

impl Data {
    const SIZE:usize = 8 + 1 + 32;
}

