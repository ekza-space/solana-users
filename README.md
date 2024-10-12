# User Profiles Smart Contract

This Solana-based smart contract, built with the Anchor framework, enables users to create and manage profiles, follow others, and maintain a registry of all users. Designed for scalability, future iterations will introduce more advanced features. It serves as a foundational engine for building decentralized communities.

### [Interact with Our 3D WebGL DApp Demo on Devnet](https://users.ekza.io/)
https://github.com/user-attachments/assets/75ed0755-f122-42b4-98a6-2df22bea3dbf

## User Profile Account Structure
The `UserProfile` struct represents a user's profile in the Solana program. It includes essential information about the user, such as their identity, links, and their social connections. This data is stored on-chain and can be referenced by other users.

```rust
#[account]
pub struct UserProfile {
    pub owner: Pubkey,              // The public key of the profile owner
    pub nickname: String,           // User's nickname
    pub description: String,        // Short bio or description
    pub twitter_link: String,       // Link to Twitter profile
    pub website_link: String,       // Personal or business website
    pub email: String,              // User's email address
    pub registration_time: i64,     // UNIX timestamp of profile registration
    pub following: Vec<Pubkey>,     // List of public keys of followed users
    pub pic: String,                // IPFS hash for profile picture
    pub avatar: String,             // IPFS hash for avatar
}
```
The total maximum size of the UserProfile account at the moment is:

```rust
const MAX_SIZE: usize = 32          // owner
    + 8                             // registration_time
    + 4 + 32                        // nickname
    + 4 + 256                       // description
    + 4 + 64                        // twitter_link
    + 4 + 64                        // website_link
    + 4 + 64                        // email
    + 4 + 64                        // pic (IPFS hash)
    + 4 + 64                        // avatar (IPFS hash)
    + 4 + (32 * 32);                // following

    // total: 1,704 bytes
```

## Features

- **Initialize User List**: Create a central account to store a list of all users.
- **Create User Profile**: Users can create a personal profile with information like nickname, description, Twitter link, and website link.
- **Follow Users**: Users can follow other users, maintaining a list of accounts they are following.
- **Update Profile**: Users can update their profile information at any time.
- **Delete Profile**: Users can delete their profiles, which also removes them from the global users list.

## Future Enhancements

- **Account Partitions**: Add account partitions to handle more users efficiently by splitting data across multiple accounts to support a larger user base.
- **Subscription with Donation**: Add a feature that allows users to subscribe to others and make donations either once or on a monthly basis.

## Getting Started

### Prerequisites
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) installed and configured.
- [Anchor CLI](https://www.anchor-lang.com/docs/installation) installed and set up.
- Node.js and npm installed.

### Development
run local node ([if meet problem with test validator](https://github.com/solana-labs/solana/issues/28899#issuecomment-1694152935))
```
% solana config set --url localhost # or devnet
% anchor build 
% anchor deploy
% anchor test
```
