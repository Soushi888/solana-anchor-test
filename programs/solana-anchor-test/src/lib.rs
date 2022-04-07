mod identity;

use anchor_lang::prelude::*;

declare_id!("HbKWMJo4U4omK8z2jkVvc569bGCNU1Ckbs1B2WYHFsDd");

#[program]
pub mod anchor_test {
    use super::*;

    /// Permet à un utilisateur sans identité de créer son identité
    pub fn create_identity(
        ctx: Context<Initialize>,
        first_name: String,
        last_name: String,
        username: String,
        birth: i64,
        mail: Option<String>,
    ) -> Result<()> {
        // Check des infos fournit par l'utilisateur
        require_gte!(identity::Identity::MAX_STRING_SIZE, first_name.len());
        require_gte!(identity::Identity::MAX_STRING_SIZE, last_name.len());
        require_gte!(identity::Identity::MAX_STRING_SIZE, username.len());
        if mail.is_some() {
            require_gte!(identity::Identity::MAX_STRING_SIZE, mail.as_ref().unwrap().len());
        }

        // Enregistrement des données dans notre account Identity
        let user_identity = &mut ctx.accounts.identity;
        user_identity.first_name = first_name;
        user_identity.last_name = last_name;
        user_identity.birth = birth;
        user_identity.mail = mail;
        user_identity.created = Clock::get().unwrap().unix_timestamp;

        Ok(())
    }

    /// Permet à un utilisateur de mettre à jour son prénom
    pub fn update_name(ctx: Context<Initialize>, first_name: String) -> Result<()> {
        // TODO
        Ok(())
    }

    /// Permet à un utilisateur de mettre à jour son pseudonyme
    pub fn update_username(ctx: Context<Initialize>, username: String) -> Result<()> {
        // TODO
        Ok(())
    }

    /// Permet à un utilisateur de mettre à jour ou supprimer son mail
    pub fn update_mail(ctx: Context<Initialize>, mail: Option<String>) -> Result<()> {
        // TODO
        Ok(())
    }

    /// Permet à un utilisateur ayant une identité depuis plus de 2 ans
    /// de supprimer son identité
    pub fn delete_identity(ctx: Context<Initialize>) -> Result<()> {
        // TODO
        Ok(())
    }

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

