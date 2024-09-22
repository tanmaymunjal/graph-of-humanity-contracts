import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { GraphOfHumanity } from "../target/types/graph_of_humanity";
import { rpcConfig } from "./test_config";
import { create_keypair, get_pda_from_seeds } from "./utils";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  getAssociatedTokenAddress,
  createMint,
  mintTo,
  transfer,
  createAssociatedTokenAccount,
} from "@solana/spl-token";
import { Orao,networkStateAccountAddress,InitBuilder,randomnessAccountAddress } from "@orao-network/solana-vrf";
import { PublicKey, LAMPORTS_PER_SOL, Keypair} from "@solana/web3.js";

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
      6
    );

    const signerTokenAddr = await createAssociatedTokenAccount(
      connection,
      initializeSigner,
      mintAddress,
      initializeSigner.publicKey
    );

    await mintTo(
      connection,
      initializeSigner,
      mintAddress,
      signerTokenAddr,
      initializeSigner,
      30 * Math.pow(10, 6)
    );

    let treasury = await get_pda_from_seeds([
      Buffer.from("treasury")
    ])
    
    let treasury_token_account = await getAssociatedTokenAddress(mintAddress,treasury,true);
    let member = await get_pda_from_seeds([
      initializeSigner.publicKey.toBuffer(),
      Buffer.from("member")
    ])

    let citizenship_appl = await get_pda_from_seeds([
      member.toBuffer(),
      Buffer.from("Welcome to my world!"),
      Buffer.from("citizenship_appl")
    ])

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
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
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
        systemProgram: web3.SystemProgram.programId
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
        systemProgram: web3.SystemProgram.programId
      })
      .signers([global.memberCreator])
      .rpc(rpcConfig);
    global.citizenshipAppl = citizenshipAppl;
  });

  it("Edit bio", async () => {
    await program.methods
      .editBio("life imprisonment")
      .accounts({
        memberCreator: global.memberCreator.publicKey,
        member: global.member,
        systemProgram: web3.SystemProgram.programId
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
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
      })
      .signers([global.user])
      .rpc(rpcConfig);
  });

  it("Fund appl", async () => {
    const signerTokenAddr = await createAssociatedTokenAccount(
      connection,
      global.memberCreator,
      global.usdcMint,
      global.memberCreator.publicKey
    );

    await mintTo(
      connection,
      global.memberCreator,
      global.usdcMint,
      signerTokenAddr,
      global.user,
      30 * Math.pow(10, 6)
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
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
      })
      .signers([global.memberCreator])
      .rpc(rpcConfig);
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
        crankerMember: global.initialMember,
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
    });
  });
