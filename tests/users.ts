import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, Keypair } from "@solana/web3.js";
import { assert } from "chai";
import { UserProfiles } from "../target/types/user_profiles";

describe("User Profiles", () => {
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  const program = anchor.workspace.UserProfiles as Program<UserProfiles>;

  let usersListPDA: PublicKey;
  let userProfilePDA: PublicKey;
  let user = provider.wallet.publicKey;

  const nickname = "TestUser";
  const description = "This is a test user";
  const twitterLink = "https://twitter.com/test";
  const websiteLink = "https://test.com";
  const email = "test@test.com";

  it("Initializes the UsersList", async () => {
    // Find PDA for UsersList
    const [usersListPDA, bump] = PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode("ekza_users_list"),
        program.programId.toBuffer(),
      ],
      program.programId
    );
    console.log("UsersList PDA:", usersListPDA.toBase58());

    // Call initialize instruction
    await program.methods
      .initialize()
      .accounts({
        usersList: usersListPDA,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    // Fetch UsersList account and check that it is initialized
    const usersListAccount = await program.account.usersList.fetch(
      usersListPDA
    );
    assert.ok(usersListAccount.users.length === 0);
  });

  it("Creates a new user profile", async () => {
    // Ensure the `usersListPDA` is fetched, but no need to reinitialize it
    [usersListPDA] = PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode("ekza_users_list"),
        program.programId.toBuffer(),
      ],
      program.programId
    );

    // Find PDA for UserProfile
    [userProfilePDA] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("ekza_user_profile"), user.toBuffer()],
      program.programId
    );

    // Ensure that `usersList` is included in accounts
    await program.methods
      .createProfile(nickname, description, twitterLink, websiteLink, email)
      .accounts({
        userProfile: userProfilePDA,
        usersList: usersListPDA, // Refer to the existing usersList PDA
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    // Fetch UserProfile account and check the data
    const userProfileAccount = await program.account.userProfile.fetch(
      userProfilePDA
    );
    assert.ok(userProfileAccount.owner.equals(provider.wallet.publicKey));
    assert.equal(userProfileAccount.nickname, nickname);
    assert.equal(userProfileAccount.description, description);
    assert.equal(userProfileAccount.twitterLink, twitterLink);
    assert.equal(userProfileAccount.websiteLink, websiteLink);
    assert.equal(userProfileAccount.email, email);
  });

  // Other tests go here (follow, update, delete) following the same approach
});
