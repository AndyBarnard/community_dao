import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { CommunityDao } from "../target/types/community_dao";

describe("community_dao", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.CommunityDao as Program<CommunityDao>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
