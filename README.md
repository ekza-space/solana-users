# User Profiles Smart Contract

This is a Solana-based smart contract written using the Anchor framework. It allows users to create and manage their profiles, follow other users, and store a list of all registered users. The program is designed for scalability, with plans to add more advanced features in future iterations.

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

### Build and Test

1. **Build the Program**:
   ```sh
   anchor build


### Development
run local node ([if meet problem with test validator](https://github.com/solana-labs/solana/issues/28899#issuecomment-1694152935))
```
solana-test-validator
solana config set --url localhost
anchor build 
anchor deploy
anchor test
```