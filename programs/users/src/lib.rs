mod accs;
use accs::*;
use anchor_lang::prelude::*;

declare_id!("Dpn8XGzXTGErx71SDLxuzVCDwJDKF79sE42r9qoY5Wpf");

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
        email: String,
        pic: String,    // field for IPFS hash
        avatar: String, // field for IPFS hash
    ) -> Result<()> {
        // Fetch the current timestamp from the Clock sysvar
        let clock = Clock::get()?;

        let profile = &mut ctx.accounts.user_profile;
        profile.owner = ctx.accounts.user.key();
        profile.nickname = nickname;
        profile.description = description;
        profile.twitter_link = twitter_link;
        profile.website_link = website_link;
        profile.email = email;
        profile.registration_time = clock.unix_timestamp;
        profile.following = Vec::new();
        profile.pic = pic;
        profile.avatar = avatar;

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
        nickname: Option<String>, // Use Option, don't update if None
        description: Option<String>,
        twitter_link: Option<String>,
        website_link: Option<String>,
        email: Option<String>,
        pic: Option<String>,
        avatar: Option<String>,
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
        if let Some(new_email) = email {
            profile.email = new_email;
        }
        if let Some(new_pic) = pic {
            profile.pic = new_pic;
        }
        if let Some(new_avatar) = avatar {
            profile.avatar = new_avatar;
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

#[error_code]
pub enum MyError {
    #[msg("Access denied")]
    Unauthorized,
    #[msg("User list is full")]
    UserListFull,
    #[msg("Invalid email format")]
    InvalidEmail,
    #[msg("Email length exceeds maximum allowed")]
    EmailTooLong,
}
