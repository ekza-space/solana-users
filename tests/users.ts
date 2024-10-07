import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Users } from "../target/types/users";
import { assert } from "chai";

describe("users", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Users as Program<Users>;

  let usersListPda: anchor.web3.PublicKey;
  let userKeypair = anchor.web3.Keypair.generate();

  before(async () => {
    // Derive PDA for the users list
    [usersListPda] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("ekza_users_list"), program.programId.toBuffer()],
      program.programId
    );

    // Airdrop SOL to user for testing
    const tx = await provider.connection.requestAirdrop(
      userKeypair.publicKey,
      anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(tx);
  });

  it("Initializes the users list", async () => {
    await program.methods
      .initialize()
      .accounts({
        usersList: usersListPda,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    // Fetch the users list account
    const usersListAccount = await program.account.usersList.fetch(
      usersListPda
    );
    assert.equal(
      usersListAccount.users.length,
      0,
      "Users list should be empty initially"
    );
  });

  it("Creates a user profile", async () => {
    // Derive PDA for the user profile
    const [userProfilePda] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("ekza_user_profile"), userKeypair.publicKey.toBuffer()],
      program.programId
    );

    // Create a user profile
    await program.methods
      .createProfile("nickname", "description", "twitter_link", "website_link")
      .accounts({
        userProfile: userProfilePda,
        usersList: usersListPda,
        user: userKeypair.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([userKeypair])
      .rpc();

    // Fetch the user profile account
    const userProfileAccount = await program.account.userProfile.fetch(
      userProfilePda
    );
    assert.equal(
      userProfileAccount.owner.toString(),
      userKeypair.publicKey.toString(),
      "Owner should match user public key"
    );
    assert.equal(
      userProfileAccount.nickname,
      "nickname",
      "Nickname should match"
    );
  });
});
