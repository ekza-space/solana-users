use anchor_lang::prelude::*;

declare_id!("CrxQKgLZU7bnm5yPsn4tMvnK9qBWDCtzoz3NCvk39eV3");

// Constants for PDA seeds
pub const USERS_LIST_SEED: &[u8] = b"ekza_users_list";
pub const USER_PROFILE_SEED: &[u8] = b"ekza_user_profile";

#[program]
pub mod user_profiles {
    use super::*;

    // Initialization of the central account for storing the list of users
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let users_list = &mut ctx.accounts.users_list;
        users_list.users = Vec::new();
        Ok(())
    }

    // Creating a user profile
    pub fn create_profile(
        ctx: Context<CreateProfile>,
        nickname: String,
        description: String,
        twitter_link: String,
        website_link: String,
    ) -> Result<()> {
        let profile = &mut ctx.accounts.user_profile;
        profile.owner = ctx.accounts.user.key();
        profile.nickname = nickname;
        profile.description = description;
        profile.twitter_link = twitter_link;
        profile.website_link = website_link;
        profile.following = Vec::new();

        // Adding the user to the general list
        let users_list = &mut ctx.accounts.users_list;
        require!(
            users_list.users.len() < UsersList::MAX_USERS,
            MyError::UserListFull
        );
        users_list.users.push(ctx.accounts.user.key());

        Ok(())
    }

    // Following another user
    pub fn follow_user(ctx: Context<FollowUser>, target_user: Pubkey) -> Result<()> {
        let profile = &mut ctx.accounts.user_profile;
        require_keys_eq!(
            profile.owner,
            ctx.accounts.user.key(),
            MyError::Unauthorized
        );

        // Adding the address to the following list if it is not already there
        if !profile.following.contains(&target_user) {
            profile.following.push(target_user);
        }

        Ok(())
    }

    pub fn update_profile(
        ctx: Context<UpdateProfile>,
        nickname: Option<String>, // Use Option, don't update is None
        description: Option<String>,
        twitter_link: Option<String>,
        website_link: Option<String>,
    ) -> Result<()> {
        let profile = &mut ctx.accounts.user_profile;
        require_keys_eq!(
            profile.owner,
            ctx.accounts.user.key(),
            MyError::Unauthorized
        );

        // Update fields if they are not None
        if let Some(new_nickname) = nickname {
            profile.nickname = new_nickname;
        }
        if let Some(new_description) = description {
            profile.description = new_description;
        }
        if let Some(new_twitter_link) = twitter_link {
            profile.twitter_link = new_twitter_link;
        }
        if let Some(new_website_link) = website_link {
            profile.website_link = new_website_link;
        }

        Ok(())
    }

    // Deleting a user profile
    pub fn delete_profile(ctx: Context<DeleteProfile>) -> Result<()> {
        let profile = &mut ctx.accounts.user_profile;
        require_keys_eq!(
            profile.owner,
            ctx.accounts.user.key(),
            MyError::Unauthorized
        );

        // Removing the user from the general list
        let users_list = &mut ctx.accounts.users_list;
        if let Some(pos) = users_list
            .users
            .iter()
            .position(|&x| x == ctx.accounts.user.key())
        {
            users_list.users.remove(pos);
        }

        // Closing the profile account (funds are returned to the user)
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + UsersList::MAX_SIZE,
        seeds = [USERS_LIST_SEED, crate::ID.as_ref()],
        bump,
    )]
    pub users_list: Account<'info, UsersList>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateProfile<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + UserProfile::MAX_SIZE,
        seeds = [USER_PROFILE_SEED, user.key().as_ref()],
        bump,
    )]
    pub user_profile: Account<'info, UserProfile>,
    #[account(mut, seeds = [USERS_LIST_SEED], bump)]
    pub users_list: Account<'info, UsersList>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateProfile<'info> {
    #[account(
        mut,
        seeds = [USER_PROFILE_SEED, user.key().as_ref()],
        bump,
        has_one = owner // This ensures that the profile owner matches the 'owner'
    )]
    pub user_profile: Account<'info, UserProfile>,
    pub owner: AccountInfo<'info>, // Owner verification field
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct FollowUser<'info> {
    #[account(
        mut,
        seeds = [USER_PROFILE_SEED, user.key().as_ref()],
        bump,
        has_one = owner // This ensures the user_profile's owner is the same as 'owner' account
    )]
    pub user_profile: Account<'info, UserProfile>,
    pub owner: AccountInfo<'info>, // Add this field to satisfy the 'has_one' requirement
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeleteProfile<'info> {
    #[account(
        mut,
        close = user,
        seeds = [USER_PROFILE_SEED, user.key().as_ref()],
        bump,
        has_one = owner // This ensures the user_profile's owner is the same as 'owner' account
    )]
    pub user_profile: Account<'info, UserProfile>,
    pub owner: AccountInfo<'info>, // Add this field to satisfy the 'has_one' requirement
    #[account(mut, seeds = [USERS_LIST_SEED], bump)]
    pub users_list: Account<'info, UsersList>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[account]
pub struct UserProfile {
    pub owner: Pubkey,
    pub nickname: String,
    pub description: String,
    pub twitter_link: String,
    pub website_link: String,
    pub following: Vec<Pubkey>, // List of addresses that the user is following
}

impl UserProfile {
    // Maximum field sizes for calculating the account size
    const MAX_NICKNAME_LEN: usize = 32;
    const MAX_DESCRIPTION_LEN: usize = 256;
    const MAX_TWITTER_LEN: usize = 64;
    const MAX_WEBSITE_LEN: usize = 64;
    const MAX_FOLLOWING_LEN: usize = 32; // Maximum number of follows
    const MAX_SIZE: usize = 32 // owner
        + 4 + Self::MAX_NICKNAME_LEN // nickname
        + 4 + Self::MAX_DESCRIPTION_LEN // description
        + 4 + Self::MAX_TWITTER_LEN // twitter_link
        + 4 + Self::MAX_WEBSITE_LEN // website_link
        + 4 + (32 * Self::MAX_FOLLOWING_LEN); // following
}

#[account]
pub struct UsersList {
    pub users: Vec<Pubkey>,
}

impl UsersList {
    const MAX_USERS: usize = 250; // Maximum number of users
    const MAX_SIZE: usize = 4 + (32 * Self::MAX_USERS); // 4 bytes for length prefix
}

#[error_code]
pub enum MyError {
    #[msg("Access denied")]
    Unauthorized,
    #[msg("User list is full")]
    UserListFull,
}
