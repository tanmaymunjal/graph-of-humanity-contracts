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
  getOrCreateAssociatedTokenAccount,
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

    const tx = await program.methods.initialize("Welcome to my world!")
      .accounts({
        initializer: initializeSigner.publicKey,
        usdcMint: mintAddress  
      })
      .signers([initializeSigner])
      .rpc(rpcConfig);
    console.log("Your transaction signature", tx);
    global.user = initializeSigner;
    global.usdcMint = mintAddress;
  });

  it("Become member!", async () => {
    await program.methods.registerMember("Tanmay","https://p.ip.fi/_dQK").accounts({
      memberCreator: global.user.publicKey
    }).signers([global.user]).rpc(rpcConfig);
  })
});
