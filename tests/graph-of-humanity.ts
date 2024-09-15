import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
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

describe("graph-of-humanity", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.GraphOfHumanity as Program<GraphOfHumanity>;
  const { web3 } = anchor;
  const {
    provider: { connection },
  } = program;
  let global: any = {};
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

    const tx = await program.methods
      .initialize("Welcome to my world!")
      .accounts({
        initializer: initializeSigner.publicKey,
        usdcMint: mintAddress,
      })
      .signers([initializeSigner])
      .rpc(rpcConfig);
    console.log("Your transaction signature", tx);
    global.user = initializeSigner;
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
      .accountsPartial({
        memberCreator: global.memberCreator.publicKey,
        memberVoucher: global.user.publicKey,
        citizenshipAppl: citizenshipAppl,
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
      })
      .signers([global.memberCreator])
      .rpc(rpcConfig);
  });

  it("Fund voucher", async () => {
    await program.methods
      .fundVoucher()
      .accountsPartial({
        memberVoucher: global.user.publicKey,
        citizenshipAppl: global.citizenshipAppl,
        usdcMint: global.usdcMint,
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
      .accountsPartial({
        memberCreator: global.memberCreator.publicKey,
        citizenshipAppl: global.citizenshipAppl,
        usdcMint: global.usdcMint,
      })
      .signers([global.memberCreator])
      .rpc(rpcConfig);
  });
});
