import {
  Connection,
  LAMPORTS_PER_SOL,
  Transaction,
  SystemProgram,
  PublicKey,
  sendAndConfirmTransaction,
  Keypair,
} from '@solana/web3.js';
import wallet from './dev-wallet.json';

const from = Keypair.fromSecretKey(new Uint8Array(wallet));
const connection = new Connection('https://api.devnet.solana.com');
const to = new PublicKey('9aqJDu144RvEdowiz8h7T48Un4cWssDaRV9Mt78B8BVq');

const transfer = async () => {
  try {
    const tx = new Transaction().add(
      SystemProgram.transfer({
        fromPubkey: from.publicKey,
        toPubkey: to,
        lamports: LAMPORTS_PER_SOL / 100,
      })
    );
    tx.recentBlockhash = (
      await connection.getRecentBlockhash('confirmed')
    ).blockhash;
    tx.feePayer = from.publicKey;
    const signature = await sendAndConfirmTransaction(connection, tx, [from]);
    console.log(`Success! Check out your TX here:
        https://explorer.solana.com/tx/${signature}?cluster=devnet`);
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
};

// transfer();

const transferAll = async () => {
  try {
    const balance = await connection.getBalance(from.publicKey);
    const tx = new Transaction().add(
      SystemProgram.transfer({
        fromPubkey: from.publicKey,
        toPubkey: to,
        lamports: balance,
      })
    );
    tx.recentBlockhash = (
      await connection.getRecentBlockhash('confirmed')
    ).blockhash;
    tx.feePayer = from.publicKey;
    const fee =
      (await connection.getFeeForMessage(tx.compileMessage(), 'confirmed'))
        .value || 0;
    tx.instructions.pop();
    tx.add(
      SystemProgram.transfer({
        fromPubkey: from.publicKey,
        toPubkey: to,
        lamports: balance - fee,
      })
    );
    const signature = await sendAndConfirmTransaction(connection, tx, [from]);
    console.log(`Success! Check out your TX here:
        https://explorer.solana.com/tx/${signature}?cluster=devnet`);
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
};
transferAll();
