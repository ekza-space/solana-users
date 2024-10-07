use anchor_lang::prelude::*;

declare_id!("CrxQKgLZU7bnm5yPsn4tMvnK9qBWDCtzoz3NCvk39eV3");

// Constants for PDA seeds
pub const PARTITIONS_LIST_SEED: &[u8] = b"partitions_list";
pub const PARTITION_SEED: &[u8] = b"partition";
pub const USER_PROFILE_SEED: &[u8] = b"user_profile";

#[program]
pub mod user_profiles {
    use super::*;

    // Initialization of the root account for storing the list of partitions
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let partitions_list = &mut ctx.accounts.partitions_list;
        partitions_list.partitions = Vec::new();
        Ok(())
    }

    // Creating a new partition (to be called when existing partitions are full)
    pub fn create_partition(ctx: Context<CreatePartition>) -> Result<()> {
        let partitions_list = &mut ctx.accounts.partitions_list;
        let partition = &mut ctx.accounts.partition;

        // Assign partition id
        let partition_id = partitions_list.partitions.len() as u64;
        partition.id = partition_id;
        partition.users = Vec::new();

        // Add the partition's key to the partitions list
        partitions_list.partitions.push(partition.key());

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
        let partitions_list = &mut ctx.accounts.partitions_list;
        let partition = &mut ctx.accounts.partition;
        let user_profile = &mut ctx.accounts.user_profile;

        // Check if the partition is filled
        if partition.users.len() >= Partition::MAX_USERS {
            // Check if the partition limit is exceeded
            if partitions_list.partitions.len() >= PartitionsList::MAX_PARTITIONS {
                return Err(MyError::MaxPartitionsReached.into());
            }

            // Create a new partition
            let _new_partition = Partition::default(); // Initialize a new partition account here
            partition.id = partitions_list.partitions.len() as u64;
            partition.users = Vec::new();
            partitions_list.partitions.push(partition.key());
        }

        // Initialize the user profile
        user_profile.owner = ctx.accounts.user.key();
        user_profile.nickname = nickname;
        user_profile.description = description;
        user_profile.twitter_link = twitter_link;
        user_profile.website_link = website_link;
        user_profile.following = Vec::new();
        user_profile.partition_id = partition.id;
        user_profile.registration_timestamp = Clock::get()?.unix_timestamp;

        // Add the user to the partition
        partition.users.push(ctx.accounts.user.key());

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

        // Add the address to the following list if it is not already there
        if !profile.following.contains(&target_user) {
            profile.following.push(target_user);
        }

        Ok(())
    }

    pub fn update_profile(
        ctx: Context<UpdateProfile>,
        nickname: Option<String>,
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
        let user_profile = &ctx.accounts.user_profile;
        let partition = &mut ctx.accounts.partition;

        require_keys_eq!(
            user_profile.owner,
            ctx.accounts.user.key(),
            MyError::Unauthorized
        );

        // Remove the user from the partition
        if let Some(pos) = partition
            .users
            .iter()
            .position(|&x| x == ctx.accounts.user.key())
        {
            partition.users.remove(pos);
        } else {
            return Err(MyError::UserNotFoundInPartition.into());
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + PartitionsList::MAX_SIZE,
        seeds = [PARTITIONS_LIST_SEED],
        bump,
    )]
    pub partitions_list: Account<'info, PartitionsList>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreatePartition<'info> {
    #[account(
        mut,
        seeds = [PARTITIONS_LIST_SEED],
        bump,
    )]
    pub partitions_list: Account<'info, PartitionsList>,
    #[account(
        init,
        payer = payer,
        space = 8 + Partition::MAX_SIZE,
        seeds = [PARTITION_SEED, (partitions_list.partitions.len() as u64).to_le_bytes().as_ref()],
        bump,
    )]
    pub partition: Account<'info, Partition>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateProfile<'info> {
    #[account(
        mut,
        seeds = [PARTITIONS_LIST_SEED],
        bump,
    )]
    pub partitions_list: Account<'info, PartitionsList>,
    #[account(
        mut,
        seeds = [PARTITION_SEED, partition.id.to_le_bytes().as_ref()],
        bump,
    )]
    pub partition: Account<'info, Partition>,
    #[account(
        init,
        payer = user,
        space = 8 + UserProfile::MAX_SIZE,
        seeds = [USER_PROFILE_SEED, user.key().as_ref()],
        bump,
    )]
    pub user_profile: Account<'info, UserProfile>,
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
        has_one = owner
    )]
    pub user_profile: Account<'info, UserProfile>,
    pub owner: Signer<'info>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct FollowUser<'info> {
    #[account(
        mut,
        seeds = [USER_PROFILE_SEED, user.key().as_ref()],
        bump,
        has_one = owner
    )]
    pub user_profile: Account<'info, UserProfile>,
    pub owner: Signer<'info>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeleteProfile<'info> {
    #[account(
        mut,
        close = user,
        seeds = [USER_PROFILE_SEED, user.key().as_ref()],
        bump,
    )]
    pub user_profile: Account<'info, UserProfile>,
    #[account(
        mut,
        seeds = [PARTITION_SEED, user_profile.partition_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub partition: Account<'info, Partition>,
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
    pub following: Vec<Pubkey>,
    pub partition_id: u64,
    pub registration_timestamp: i64,
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
        + 8 // partition_id
        + 8 // registration_timestamp
        + 4 + (32 * Self::MAX_FOLLOWING_LEN); // following
}

#[account]
pub struct PartitionsList {
    pub partitions: Vec<Pubkey>,
}

impl PartitionsList {
    const MAX_PARTITIONS: usize = 250; // Maximum number of partitions
    const MAX_SIZE: usize = 4 + (32 * Self::MAX_PARTITIONS); // 4 bytes for length prefix
}

#[account]
pub struct Partition {
    pub id: u64,
    pub users: Vec<Pubkey>,
}

impl Partition {
    const MAX_USERS: usize = 250; // Maximum number of users per partition
    const MAX_SIZE: usize = 8 // id
        + 4 + (32 * Self::MAX_USERS); // users
}

impl Default for Partition {
    fn default() -> Self {
        Self {
            id: 0, // or other value
            users: Vec::new(),
        }
    }
}

#[error_code]
pub enum MyError {
    #[msg("Access denied")]
    Unauthorized,
    #[msg("User not found in partition")]
    UserNotFoundInPartition,
    #[msg("Max partitions reached")]
    MaxPartitionsReached,
}
