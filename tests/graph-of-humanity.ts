import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { GraphOfHumanity } from "../target/types/graph_of_humanity";
import { rpcConfig } from "./test_config";
import { create_keypair, get_pda_from_seeds, sleep, getReturnLog } from "./utils";
import * as borsh from "borsh";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddress,
  createMint,
  mintTo,
  createAssociatedTokenAccount,
  TOKEN_2022_PROGRAM_ID,
} from "@solana/spl-token";
import {
  Orao,
  networkStateAccountAddress,
  InitBuilder,
  FulfillBuilder,
  randomnessAccountAddress,
} from "@orao-network/solana-vrf";
import nacl from "tweetnacl";
import { LAMPORTS_PER_SOL, Keypair } from "@solana/web3.js";
import assert from "assert";

describe("graph-of-humanity", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.GraphOfHumanity as Program<GraphOfHumanity>;
  const { web3 } = anchor;
  const {
    provider: { connection },
  } = program;
  let global: any = {};
  const fulfillmentAuthority = Keypair.generate();
  // Initialize ORAO VRF client
  const vrf = new Orao(program.provider);
  const vrfTreasury = Keypair.generate();

  // This helper will fulfill randomness for our test VRF.
  async function emulateFulfill(seed: Buffer) {
    let signature = nacl.sign.detached(seed, fulfillmentAuthority.secretKey);
    await new FulfillBuilder(vrf, seed).rpc(
      fulfillmentAuthority.publicKey,
      signature
    );
  }

  before(async () => {
    // Initialize test VRF
    const fee = 2 * LAMPORTS_PER_SOL;
    const fulfillmentAuthorities = [fulfillmentAuthority.publicKey];
    const configAuthority = Keypair.generate();

    await new InitBuilder(
      vrf,
      configAuthority.publicKey,
      vrfTreasury.publicKey,
      fulfillmentAuthorities,
      new BN(fee)
    ).rpc();
  });

  it("Is initialized!", async () => {
    // Add your test here.
    const initializeSigner = await create_keypair();

    const mintAddress = await createMint(
      connection,
      initializeSigner,
      initializeSigner.publicKey,
      initializeSigner.publicKey,
      6,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID
    );

    const signerTokenAddr = await createAssociatedTokenAccount(
      connection,
      initializeSigner,
      mintAddress,
      initializeSigner.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID
    );

    await mintTo(
      connection,
      initializeSigner,
      mintAddress,
      signerTokenAddr,
      initializeSigner,
      30 * Math.pow(10, 12),
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID
    );

    let treasury = await get_pda_from_seeds([Buffer.from("treasury")]);

    let treasury_token_account = await getAssociatedTokenAddress(
      mintAddress,
      treasury,
      true,
      TOKEN_2022_PROGRAM_ID
    );

    let member = await get_pda_from_seeds([
      initializeSigner.publicKey.toBuffer(),
      Buffer.from("member"),
    ]);

    let citizenship_appl = await get_pda_from_seeds([
      member.toBuffer(),
      Buffer.from("Welcome to my world!"),
      Buffer.from("citizenship_appl"),
    ]);

    const tx = await program.methods
      .initialize("Welcome to my world!")
      .accounts({
        initializer: initializeSigner.publicKey,
        treasury: treasury,
        treasuryTokenAccount: treasury_token_account,
        member: member,
        citizenshipAppl: citizenship_appl,
        usdcMint: mintAddress,
        systemProgram: web3.SystemProgram.programId,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .signers([initializeSigner])
      .rpc(rpcConfig);
    console.log("Your transaction signature", tx);
    global.user = initializeSigner;
    global.treasury = treasury;
    global.treasuryTokenAccount = treasury_token_account;
    global.initialMember = member;
    global.initialCitizen = citizenship_appl;
    global.signerTokenAddr = signerTokenAddr;
    global.usdcMint = mintAddress;
  });

  it("Become member!", async () => {
    let memberCreator = await create_keypair();
    let member = await get_pda_from_seeds([
      memberCreator.publicKey.toBuffer(),
      Buffer.from("member"),
    ]);
    await program.methods
      .registerMember("Tanmay", "https://p.ip.fi/_dQK")
      .accounts({
        memberCreator: memberCreator.publicKey,
        member: member,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([memberCreator])
      .rpc(rpcConfig);
    global.memberCreator = memberCreator;
    global.member = member;
  });

  it("Apply citizenship", async () => {
    const citizenshipAppl = await get_pda_from_seeds([
      global.member.toBuffer(),
      Buffer.from("1"),
      Buffer.from("citizenship_appl"),
    ]);
    await program.methods
      .applyCitizenship("1", "https://p.ip.fi/_dQK", null)
      .accounts({
        memberCreator: global.memberCreator.publicKey,
        memberVoucher: global.user.publicKey,
        memberVoucherAccount: global.initialMember,
        member: global.member,
        citizenshipAppl: citizenshipAppl,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([global.memberCreator])
      .rpc(rpcConfig);
    global.citizenshipAppl = citizenshipAppl;
  });

  it("Edit bio", async () => {
    await program.methods
      .editUser("life imprisonment", "Super Mario")
      .accounts({
        memberCreator: global.memberCreator.publicKey,
        member: global.member,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([global.memberCreator])
      .rpc(rpcConfig);
  });

  it("Fund voucher", async () => {
    await program.methods
      .fundVoucher()
      .accounts({
        memberVoucher: global.user.publicKey,
        citizenshipAppl: global.citizenshipAppl,
        memberVoucherAccount: global.initialMember,
        memberVoucherTokenAccount: global.signerTokenAddr,
        usdcMint: global.usdcMint,
        treasury: global.treasury,
        treasuryTokenAccount: global.treasuryTokenAccount,
        systemProgram: web3.SystemProgram.programId,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .signers([global.user])
      .rpc(rpcConfig);
  });

  it("Fund appl", async () => {
    const signerTokenAddr = await createAssociatedTokenAccount(
      connection,
      global.memberCreator,
      global.usdcMint,
      global.memberCreator.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID
    );

    await mintTo(
      connection,
      global.memberCreator,
      global.usdcMint,
      signerTokenAddr,
      global.user,
      30 * Math.pow(10, 6),
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID
    );

    await program.methods
      .fundCitizenshipAppl()
      .accounts({
        memberCreator: global.memberCreator.publicKey,
        member: global.member,
        memberTokenAccount: signerTokenAddr,
        citizenshipAppl: global.citizenshipAppl,
        treasury: global.treasury,
        treasuryTokenAccount: global.treasuryTokenAccount,
        usdcMint: global.usdcMint,
        systemProgram: web3.SystemProgram.programId,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .signers([global.memberCreator])
      .rpc(rpcConfig);
    global.memberTokenAccount = signerTokenAddr;
  });

  it("Request random judges", async () => {
    // Generate a new force (seed) for randomness
    const force = anchor.web3.Keypair.generate().publicKey.toBuffer();

    // Calculate the randomness account address
    const randomnessAccount = randomnessAccountAddress(force);

    // Get the VRF treasury account
    const vrfConfig = networkStateAccountAddress();
    await program.methods
      .requestRandomnessVoters(Array.from(force))
      .accounts({
        cranker: global.user.publicKey,
        citizenshipAppl: global.citizenshipAppl,
        randomnessAccount: randomnessAccount,
        treasury: global.treasury,
        vrfProgram: vrf.programId,
        vrfConfig: vrfConfig,
        vrfTreasury: vrfTreasury.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([global.user])
      .rpc(rpcConfig);
    global.randomnessAccount = randomnessAccount;
    global.force = force;
  });

  it("Reveal random judges", async () => {
    let cranker = await create_keypair();
    await emulateFulfill(global.force);
    let res = await program.methods
        .revealRandomnessVoters()
        .accounts({
          cranker: cranker.publicKey,
          citizenshipAppl: global.citizenshipAppl,
          randomnessAccountData: global.randomnessAccount,
          treasury: global.treasury,
          systemProgram: web3.SystemProgram.programId,
        })
        .signers([cranker])
        .rpc(rpcConfig);
        let t = await connection.getTransaction(res, {
          commitment: "confirmed",
        });
    
        const [key, data, buffer] = getReturnLog(t);
        assert.equal(key, program.programId);
    
        const reader = new borsh.BinaryReader(buffer);
        const array = reader.readArray(() => reader.readU8());
      });
  it("Vote for user", async () => {
    let voteAcc = await get_pda_from_seeds([
      Buffer.from("vote"),
      global.user.publicKey.toBuffer(),
      global.citizenshipAppl.toBuffer(),
    ]);
    await program.methods
      .voteCitizen(true, null)
      .accounts({
        voter: global.user.publicKey,
        voterMember: global.initialMember,
        memberCitizenshipAppl: global.citizenshipAppl,
        voteAcc: voteAcc,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([global.user])
      .rpc(rpcConfig);
    global.voteAcc = voteAcc;
  });

  it("Check result", async () => {
    await sleep(5 * 1000);
    let cranker = await create_keypair();
    await program.methods
      .checkResult()
      .accounts({
        cranker: cranker.publicKey,
        member: global.member,
        memberCitizenshipAppl: global.citizenshipAppl,
        treasury: global.treasury,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([cranker])
      .rpc(rpcConfig);
  });
  it("Claim reward for voting correctly", async () => {
    await program.methods
      .claimReward()
      .accounts({
        voter: global.user.publicKey,
        voteAcc: global.voteAcc,
        voterMember: global.initialMember,
        memberCitizenshipAppl: global.citizenshipAppl,
        voterTokenAccount: global.signerTokenAddr,
        treasury: global.treasury,
        treasuryTokenAccount: global.treasuryTokenAccount,
        usdcMint: global.usdcMint,
        systemProgram: web3.SystemProgram.programId,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .signers([global.user])
      .rpc(rpcConfig);
  });
  it("Donate money", async () => {
    await program.methods
      .donateMoney(new BN(100000000000))
      .accounts({
        doner: global.user.publicKey,
        donerTokenAccount: global.signerTokenAddr,
        treasury: global.treasury,
        treasuryTokenAccount: global.treasuryTokenAccount,
        usdcMint: global.usdcMint,
        systemProgram: web3.SystemProgram.programId,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .signers([global.user])
      .rpc(rpcConfig);
  });
  it("Start distribution", async () => {
    let cranker = await create_keypair();
    let epoch = await get_pda_from_seeds([
      Buffer.from("1"),
      Buffer.from("di_epoch"),
    ]);

    await program.methods
      .startDistributionEpoch()
      .accounts({
        cranker: cranker.publicKey,
        treasury: global.treasury,
        treasuryTokenAccount: global.treasuryTokenAccount,
        usdcMint: global.usdcMint,
        epoch: epoch,
        systemProgram: web3.SystemProgram.programId,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .signers([cranker])
      .rpc(rpcConfig);
    global.epoch = epoch;
  });
  it("Request ubi randomness", async () => {
    // Generate a new force (seed) for randomness
    const force = anchor.web3.Keypair.generate().publicKey.toBuffer();
    // Calculate the randomness account address
    const randomnessAccount = randomnessAccountAddress(force);

    // Get the VRF treasury account
    const vrfConfig = networkStateAccountAddress();

    let ubi_randomness_acc = await get_pda_from_seeds([
      global.epoch.toBuffer(),
      force,
      Buffer.from("ubi_randomness_acc"),
    ]);

    let cranker = await create_keypair();
    await program.methods
      .requestUbiRandomness(Array.from(force))
      .accounts({
        cranker: cranker.publicKey,
        treasury: global.treasury,
        epoch: global.epoch,
        ubiRandomnessAcc: ubi_randomness_acc,
        randomnessAccount: randomnessAccount,
        vrfConfig: vrfConfig,
        vrfProgram: vrf.programId,
        vrfTreasury: vrfTreasury.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([cranker])
      .rpc(rpcConfig);
    await emulateFulfill(force);
    global.ubiRandomnessAcc = ubi_randomness_acc;
    global.ubi_randomness_acc_vrf = randomnessAccount;
  });
  it("Reveal ubi randomness", async () => {
    let cranker = await create_keypair();
    await program.methods
      .revealUbiRandomness()
      .accounts({
        cranker: cranker.publicKey,
        randomnessAccountData: global.ubi_randomness_acc_vrf,
        treasury: global.treasury,
        epoch: global.epoch,
        ubiRandomnessAcc: global.ubiRandomnessAcc,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([cranker])
      .rpc(rpcConfig);
  });
  it("Claim ubi", async () => {
    let claim_hashmap = await get_pda_from_seeds([
      global.user.publicKey.toBuffer(),
      global.epoch.toBuffer(),
      Buffer.from("claim_hashmap"),
    ]);
    await program.methods
      .claimUbi()
      .accounts({
        claimer: global.user.publicKey,
        claimerMemberAcc: global.initialMember,
        claimerTokenAccount: global.signerTokenAddr,
        treasury: global.treasury,
        claimHashmap: claim_hashmap,
        treasuryTokenAccount: global.treasuryTokenAccount,
        epoch: global.epoch,
        ubiRandomnessAcc: global.ubiRandomnessAcc,
        usdcMint: global.usdcMint,
        systemProgram: web3.SystemProgram.programId,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .signers([global.user])
      .rpc(rpcConfig);

    let new_claim_hashmap = await get_pda_from_seeds([
      global.memberCreator.publicKey.toBuffer(),
      global.epoch.toBuffer(),
      Buffer.from("claim_hashmap"),
    ]);
    await program.methods
      .claimUbi()
      .accounts({
        claimer: global.memberCreator.publicKey,
        claimerMemberAcc: global.member,
        claimerTokenAccount: global.memberTokenAccount,
        treasury: global.treasury,
        claimHashmap: new_claim_hashmap,
        treasuryTokenAccount: global.treasuryTokenAccount,
        epoch: global.epoch,
        ubiRandomnessAcc: global.ubiRandomnessAcc,
        usdcMint: global.usdcMint,
        systemProgram: web3.SystemProgram.programId,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .signers([global.memberCreator])
      .rpc(rpcConfig);
  });
});
