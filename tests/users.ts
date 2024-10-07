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

  it("Follows another user", async () => {
    // Create a second user keypair
    const secondUser = Keypair.generate();
    const secondUserPubkey = secondUser.publicKey;

    // Find PDA for the second user's profile
    const [secondUserProfilePDA] = PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode("ekza_user_profile"),
        secondUserPubkey.toBuffer(),
      ],
      program.programId
    );

    // Airdrop some SOL to the second user for fees
    const airdropSignature = await provider.connection.requestAirdrop(
      secondUserPubkey,
      1 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(airdropSignature);

    // Create profile for the second user
    await program.methods
      .createProfile(
        "SecondUser",
        "This is a second user",
        "",
        "",
        "second@test.com"
      )
      .accounts({
        userProfile: secondUserProfilePDA,
        usersList: usersListPDA, // Refer to the existing usersList PDA
        user: secondUserPubkey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([secondUser])
      .rpc();

    console.log("Second user's public key:", secondUserPubkey.toBase58());
    console.log("First user's profile PDA:", userProfilePDA.toBase58());
    console.log("Second user's profile PDA:", secondUserProfilePDA.toBase58());

    // Follow second user from the first user's profile
    await program.methods
      .followUser(secondUserPubkey)
      .accounts({
        userProfile: userProfilePDA, // First user's profile
        usersList: usersListPDA,
        user: provider.wallet.publicKey,
      })
      .rpc();

    // Fetch the first user's profile to check the following list
    const userProfileAccount = await program.account.userProfile.fetch(
      userProfilePDA
    );
    console.log(
      "First user's following list:",
      userProfileAccount.following.map((pk) => pk.toBase58())
    );

    // Check if the second user's public key is in the following list
    const isFollowing = userProfileAccount.following.some((key: PublicKey) =>
      key.equals(secondUserPubkey)
    );

    // Assert the second user is in the following list
    assert.ok(isFollowing, "The second user should be in the following list");
  });

  it("Updates user profile", async () => {
    // New data for the profile update
    const newNickname = "UpdatedUser";
    const newDescription = "This is an updated description";
    const newTwitterLink = "https://twitter.com/updateduser";
    const newWebsiteLink = "https://updateduser.com";
    const newEmail = "updateduser@test.com";

    // Update the first user's profile with new data
    await program.methods
      .updateProfile(
        newNickname, // Update nickname
        newDescription, // Update description
        newTwitterLink, // Update Twitter link
        newWebsiteLink, // Update website link
        newEmail // Update email
      )
      .accounts({
        userProfile: userProfilePDA,
        user: provider.wallet.publicKey,
      })
      .rpc();

    // Fetch the updated profile to verify changes
    const updatedProfile = await program.account.userProfile.fetch(
      userProfilePDA
    );

    // Assert the fields were updated correctly
    assert.equal(updatedProfile.nickname, newNickname);
    assert.equal(updatedProfile.description, newDescription);
    assert.equal(updatedProfile.twitterLink, newTwitterLink);
    assert.equal(updatedProfile.websiteLink, newWebsiteLink);
    assert.equal(updatedProfile.email, newEmail);
  });

  it("Deletes user profile", async () => {
    // Delete the user profile
    await program.methods
      .deleteProfile()
      .accounts({
        userProfile: userProfilePDA,
        usersList: usersListPDA,
        user: provider.wallet.publicKey,
      })
      .rpc();

    // Try to fetch the profile to ensure it no longer exists
    try {
      await program.account.userProfile.fetch(userProfilePDA);
      assert.fail("The profile should have been deleted, but it still exists.");
    } catch (err) {
      assert.ok(
        err.message.includes("Account does not exist"),
        "Expected error about missing account"
      );
    }

    // Fetch the users list to verify the profile was removed
    const usersListAccount = await program.account.usersList.fetch(
      usersListPDA
    );
    assert.ok(
      !usersListAccount.users.includes(provider.wallet.publicKey),
      "User was not removed from the users list."
    );
  });

  // Other tests go here following the same approach
});
