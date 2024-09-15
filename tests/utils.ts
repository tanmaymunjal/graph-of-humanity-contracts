import * as anchor from "@coral-xyz/anchor";
import { GraphOfHumanity } from "../target/types/graph_of_humanity";

// Configure the client to use the local cluster.
anchor.setProvider(anchor.AnchorProvider.env());

//testing defios workspace here
const program = anchor.workspace.GraphOfHumanity as anchor.Program<GraphOfHumanity>;
const {
  provider: { connection },
} = program;
const { web3 } = anchor;

async function create_keypair() {
  const keypair = web3.Keypair.generate();
  await connection.confirmTransaction(
    {
      signature: await connection.requestAirdrop(
        keypair.publicKey,
        web3.LAMPORTS_PER_SOL
      ),
      ...(await connection.getLatestBlockhash()),
    },
    "confirmed"
  );
  return keypair;
}

async function get_pda_from_seeds(seeds) {
  return web3.PublicKey.findProgramAddressSync(seeds, program.programId)[0];
}

export { create_keypair, get_pda_from_seeds };
