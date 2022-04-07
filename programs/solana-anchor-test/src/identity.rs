use anchor_lang::prelude::*;

#[account]
pub struct Identity {
    pub first_name: String,
    // max 128 bytes
    pub last_name: String,
    // max 128 bytes
    pub username: String,
    // max 128 bytes
    pub birth: i64,
    pub mail: Option<String>,
    // max 128 bytes
    pub created: i64,
}


impl Identity {
    pub const MAX_STRING_SIZE: usize = 128;
    pub const MAX_IDENTITY_SIZE: usize = 132 + 132 + 132 + 8 + 129 + 8;
}

#[derive(Accounts)]
pub struct CreateIdentity<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init, payer = user, space = Identity::MAX_IDENTITY_SIZE + 8)]
    pub identity: Account<'info, Identity>,
    pub system_program: SystemAccount<'info>,
}

#[derive(Accounts)]
pub struct UpdateIdentity<'info> {
    pub user: Signer<'info>,
    #[account(mut)]
    pub identity: Account<'info, Identity>
}
