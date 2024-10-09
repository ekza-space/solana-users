use anchor_lang::prelude::*;

// Constants for PDA seeds
pub const USERS_LIST_SEED: &[u8] = b"users_list";
pub const USER_PROFILE_SEED: &[u8] = b"user_profile";

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
    #[account(mut, seeds = [USERS_LIST_SEED, crate::ID.as_ref()], bump)]
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
    /// CHECK: The 'owner' account is checked by the 'has_one'
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
    /// CHECK: 'owner' is verified via 'has_one' constraint on 'user_profile'
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
    /// CHECK: 'owner' is verified via 'has_one' constraint on 'user_profile'
    pub owner: AccountInfo<'info>, // Add this field to satisfy the 'has_one' requirement
    #[account(mut, seeds = [USERS_LIST_SEED, crate::ID.as_ref()], bump)]
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
    pub email: String,
    pub registration_time: i64,
    pub following: Vec<Pubkey>, // List of addresses that the user is following
    pub pic: String,            // IPFS hash for profile picture
    pub avatar: String,         // IPFS hash for avatar
}
impl UserProfile {
    // Maximum field sizes for calculating the account size
    const MAX_NICKNAME_LEN: usize = 32;
    const MAX_DESCRIPTION_LEN: usize = 256;
    const MAX_TWITTER_LEN: usize = 64;
    const MAX_WEBSITE_LEN: usize = 64;
    const MAX_EMAIL_LEN: usize = 64;
    const MAX_PIC_LEN: usize = 64; // Assuming IPFS hash length is 64 characters
    const MAX_AVATAR_LEN: usize = 64; // Assuming IPFS hash length is 64 characters
    const MAX_FOLLOWING_LEN: usize = 32; // Maximum number of follows
    const MAX_SIZE: usize = 32 // owner
        + 8 // registration_time (i64)
        + 4 + Self::MAX_NICKNAME_LEN // nickname
        + 4 + Self::MAX_DESCRIPTION_LEN // description
        + 4 + Self::MAX_TWITTER_LEN // twitter_link
        + 4 + Self::MAX_WEBSITE_LEN // website_link
        + 4 + Self::MAX_EMAIL_LEN // email
        + 4 + Self::MAX_PIC_LEN // pic (IPFS hash)
        + 4 + Self::MAX_AVATAR_LEN // avatar (IPFS hash)
        + 4 + (32 * Self::MAX_FOLLOWING_LEN); // following
}

#[account]
pub struct UsersList {
    pub users: Vec<Pubkey>,
}

impl UsersList {
    pub const MAX_USERS: usize = 250; // Maximum number of users
    pub const MAX_SIZE: usize = 4 + (32 * Self::MAX_USERS); // 4 bytes for length prefix
}
