import { web3, workspace, getProvider, Program } from "@coral-xyz/anchor";
import { Example } from "../target/types/example";
import { createMint } from "@solana/spl-token";

describe("carrot-protocol-cpi-example", () => {
  const program = workspace.Example as Program<Example>;
  const connection = program.provider.connection;

  it("initialize", async () => {
    const carrotProgram = new web3.PublicKey(
      "CarrotwivhMpDnm27EHmRLeQ683Z1PufuqEmBZvD282s"
    );

    const authority = await newPayer(connection);
    console.log("authority: %s", authority.publicKey.toString());

    const sharesKp = web3.Keypair.generate();

    const vault = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), sharesKp.publicKey.toBuffer()],
      carrotProgram
    )[0];

    const shares = await createMint(
      connection,
      authority,
      vault,
      null,
      9,
      sharesKp,
      { skipPreflight: false }
    );
    console.log("shares: %s", shares.toString());

    const initIx = await program.methods
      .initialize()
      .accounts({
        vault,
        shares: sharesKp.publicKey,
        authority: authority.publicKey,
        systemProgram: web3.SystemProgram.programId,
        carrotProgram,
      })
      .instruction();

    const tx = new web3.Transaction().add(initIx);
    const txSig = await connection.sendTransaction(tx, [authority], {
      skipPreflight: false,
    });
    console.log("initializeTxSig: %s", txSig);
  });
});

async function newPayer(connection: web3.Connection): Promise<web3.Keypair> {
  const payer = web3.Keypair.generate();

  const txSig = await connection.requestAirdrop(
    payer.publicKey,
    web3.LAMPORTS_PER_SOL * 100
  );
  await connection.confirmTransaction(txSig, "confirmed");

  return payer;
}
