import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, Keypair } from "@solana/web3.js";
import { assert } from "chai";
import { UserProfiles } from "../target/types/user_profiles";

describe("User Profiles", () => {
  const users_list = "users_list";
  // const users_list = `users_list_${Math.random()}`;
  const user_profile = "user_profile";

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
  const pic = "QmTestProfilePic"; // Mock IPFS hash for profile picture
  const avatar = "QmTestAvatarHash"; // Mock IPFS hash for avatar

  it("Initializes the UsersList", async () => {
    const [usersListPDA, bump] = PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode(users_list),
        program.programId.toBuffer(),
      ],
      program.programId
    );
    console.log("UsersList PDA:", usersListPDA.toBase58());

    // Проверим, существует ли аккаунт
    const usersListAccount = await program.account.usersList.fetch(
      usersListPDA
    );

    if (!usersListAccount) {
      await program.methods
        .initialize()
        .accounts({
          usersList: usersListPDA,
          user: provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      const usersList = await program.account.usersList.fetch(usersListPDA);
      assert.ok(usersList.users.length === 0);
      console.log("UsersList initialized");
    } else {
      const usersList = await program.account.usersList.fetch(usersListPDA);
      assert.ok(usersList.users.length != 0);
      console.log("UsersList already exists, skipping initialization.");
    }
  });

  it("Creates a new user profile", async () => {
    // Find PDA for UsersList
    [usersListPDA] = PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode(users_list),
        program.programId.toBuffer(),
      ],
      program.programId
    );

    // Find PDA for UserProfile
    [userProfilePDA] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode(user_profile), user.toBuffer()],
      program.programId
    );

    // Call createProfile with all necessary fields
    await program.methods
      .createProfile(
        nickname,
        description,
        twitterLink,
        websiteLink,
        email,
        pic,
        avatar
      )
      .accounts({
        userProfile: userProfilePDA,
        usersList: usersListPDA,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    // Fetch UserProfile account and check the data
    const userProfileAccount = await program.account.userProfile.fetch(
      userProfilePDA
    );
    console.log("Created user profile: ", userProfileAccount);

    assert.ok(userProfileAccount.owner.equals(provider.wallet.publicKey));
    assert.equal(userProfileAccount.nickname, nickname);
    assert.equal(userProfileAccount.description, description);
    assert.equal(userProfileAccount.twitterLink, twitterLink);
    assert.equal(userProfileAccount.websiteLink, websiteLink);
    assert.equal(userProfileAccount.email, email);
    assert.equal(userProfileAccount.pic, pic);
    assert.equal(userProfileAccount.avatar, avatar);
  });

  it("Follows another user", async () => {
    // Create a second user keypair
    const secondUser = Keypair.generate();
    const secondUserPubkey = secondUser.publicKey;

    // Find PDA for the second user's profile
    const [secondUserProfilePDA] = PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode(user_profile),
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
        "second@test.com",
        "QmSecondProfilePic",
        "QmSecondAvatarHash"
      )
      .accounts({
        userProfile: secondUserProfilePDA,
        usersList: usersListPDA,
        user: secondUserPubkey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([secondUser])
      .rpc();

    // Follow second user from the first user's profile
    await program.methods
      .followUser(secondUserPubkey)
      .accounts({
        userProfile: userProfilePDA,
        user: provider.wallet.publicKey,
      })
      .rpc();

    // Fetch the first user's profile to check the following list
    const userProfileAccount = await program.account.userProfile.fetch(
      userProfilePDA
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
    const newPic = "QmUpdatedProfilePic"; // Updated mock IPFS hash for profile picture
    const newAvatar = "QmUpdatedAvatar"; // Updated mock IPFS hash for avatar

    // Update the first user's profile with new data
    await program.methods
      .updateProfile(
        newNickname,
        newDescription,
        newTwitterLink,
        newWebsiteLink,
        newEmail,
        newPic,
        newAvatar
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
    assert.equal(updatedProfile.pic, newPic);
    assert.equal(updatedProfile.avatar, newAvatar);
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
      !usersListAccount.users.some((pk) =>
        pk.equals(provider.wallet.publicKey)
      ),
      "User was not removed from the users list."
    );
  });

  it("Check all users and profile", async () => {
    const [usersListPDA, bump] = PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode(users_list),
        program.programId.toBuffer(),
      ],
      program.programId
    );

    let response = await program.account.usersList.fetch(usersListPDA);
    console.log("All registred users: ", response);
  });
});
