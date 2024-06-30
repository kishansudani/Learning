// import {
//   Keypair,
//   Connection,
//   PublicKey,
//   clusterApiUrl,
//   LAMPORTS_PER_SOL,
//   SystemProgram,
//   TransactionInstruction,
//   Transaction,
//   sendAndConfirmTransaction,
// } from "@solana/web3.js";
// import fs from "mz/fs";
// import path from "path";
// import * as borsh from "borsh";

const {
  Keypair,
  Connection,
  PublicKey,
  clusterApiUrl,
  LAMPORTS_PER_SOL,
  SystemProgram,
  TransactionInstruction,
  Transaction,
  sendAndConfirmTransaction,
} = require("@solana/web3.js");

const fs = require("mz/fs");
const path = require("path");
const { serialize, deserialize } = require("borsh");

let connection;
let payer;
let programId;
let greetedPubkey;

// const rpcUrl = "https://api.devnet.solana.com";
const rpcUrl = "http://127.0.0.1:8899";

const PROGRAM_PATH = path.resolve(__dirname, "../../dist/program");
const PROGRAM_SO_PATH = path.join(PROGRAM_PATH, "helloworld.so");
const PROGRAM_KEYPAIR_PATH = path.join(PROGRAM_PATH, "helloworld-keypair.json");

class GreetingAccount {
  counter = 0;
  constructor(fields) {
    if (fields) {
      this.counter = fields.counter;
    }
  }
}

const GreetingSchema = new Map([
  [GreetingAccount, { kind: "struct", fields: [["counter", "u32"]] }],
]);

const GREETING_SIZE = serialize(GreetingSchema, new GreetingAccount()).length;

async function establishConnection() {
  connection = new Connection(rpcUrl, "confirmed");
}

async function establishPayer() {
  payer = new Keypair();
  const publicKey = new PublicKey(payer._keypair.publicKey).toString();
  const version = await connection.getVersion();

  console.log("connection to cluster established: ", version);

  const fromAirDropSignature = await connection.requestAirdrop(
    new PublicKey(publicKey),
    2 * LAMPORTS_PER_SOL
  );

  await connection.confirmTransaction(fromAirDropSignature);

  console.log(
    "Using account",
    payer.publicKey.toBase58(),
    "containing",
    2 * LAMPORTS_PER_SOL,
    "SOL to pay for fees"
  );
}

async function checkProgram() {
  try {
    const programKeypair = await createKeypairFromFile(PROGRAM_KEYPAIR_PATH);
    programId = programKeypair.publicKey;
  } catch (err) {
    const errMsg = err.message;
    throw new Error(
      `Failed to read program keypair at '${PROGRAM_KEYPAIR_PATH}' due to error: ${errMsg}. Program may need to be deployed with \`solana program deploy dist/program/helloworld.so\``
    );
  }

  const programInfo = await connection.getAccountInfo(programId);
  if (programInfo === null) {
    if (fs.existsSync(PROGRAM_SO_PATH)) {
      throw new Error(
        "Program needs to be deployed with `solana program deploy dist/program/helloworld.so`"
      );
    } else {
      throw new Error("Program needs to be built and deployed");
    }
  } else if (!programInfo.executable) {
    throw new Error(`Program is not executable`);
  }
  console.log(`Using program ${programId.toBase58()}`);

  const GREETING_SEED = "hello";
  greetedPubkey = await PublicKey.createWithSeed(
    payer.publicKey,
    GREETING_SEED,
    programId
  );

  // Check if the greeting account has already been created
  const greetedAccount = await connection.getAccountInfo(greetedPubkey);
  if (greetedAccount === null) {
    console.log(
      "Creating account",
      greetedPubkey.toBase58(),
      "to say hello to"
    );
    const lamports = await connection.getMinimumBalanceForRentExemption(
      GREETING_SIZE
    );

    const transaction = new Transaction().add(
      SystemProgram.createAccountWithSeed({
        fromPubkey: payer.publicKey,
        basePubkey: payer.publicKey,
        seed: GREETING_SEED,
        newAccountPubkey: greetedPubkey,
        lamports,
        space: GREETING_SIZE,
        programId,
      })
    );
    await sendAndConfirmTransaction(connection, transaction, [payer]);
  }
}

async function createKeypairFromFile(filePath) {
  const secretKeyString = await fs.readFile(filePath, { encoding: "utf8" });
  const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
  return Keypair.fromSecretKey(secretKey);
}

async function sayHello() {
  console.log("Saying hello to", greetedPubkey.toBase58());

  const instruction = new TransactionInstruction({
    keys: [{ pubkey: greetedPubkey, isSigner: false, isWritable: true }],
    programId,
    data: Buffer.alloc(0),
  });

  await sendAndConfirmTransaction(
    connection,
    new Transaction().add(instruction),
    [payer]
  );
}

async function reportGreetings() {
  const accountInfo = await connection.getAccountInfo(greetedPubkey);
  if (accountInfo === null) {
    throw "Error: cannot find the greeted account";
  }
  const greeting = deserialize(
    GreetingSchema,
    GreetingAccount,
    accountInfo.data
  );
  console.log(
    greetedPubkey.toBase58(),
    "has been greeted",
    greeting.counter,
    "time(s)"
  );
}

module.exports = {
  establishConnection,
  establishPayer,
  checkProgram,
  createKeypairFromFile,
  sayHello,
  reportGreetings,
};
