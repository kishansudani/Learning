// Import Solana web3 functionalities
const {
  Connection,
  PublicKey,
  clusterApiUrl,
  Keypair,
  LAMPORTS_PER_SOL,
} = require("@solana/web3.js");

const newPair = new Keypair();

const publicKey = new PublicKey(newPair._keypair.publicKey).toString();
const privateKey = newPair._keypair.secretKey;

console.log("Public Key of the generated keypair", publicKey, privateKey);

const getWalletBalance = async () => {
  try {
    const connection = new Connection("https://api.devnet.solana.com");

    const myWallet = await Keypair.fromSecretKey(privateKey);
    const walletBalance = await connection.getBalance(
      new PublicKey(newPair.publicKey)
    );
    console.log(
      `Wallet balance: ${parseInt(walletBalance) / LAMPORTS_PER_SOL} SOL`
    );
  } catch (err) {
    console.log(err);
  }
};

const airDropSol = async () => {
  try {
    const connection = new Connection("https://api.devnet.solana.com");
    const myWallet = await Keypair.fromSecretKey(privateKey);

    console.log("Airdropping some SOL to my wallet!");
    const fromAirDropSignature = await connection.requestAirdrop(
      new PublicKey(myWallet.publicKey),
      1 * LAMPORTS_PER_SOL
    );
    await connection.confirmTransaction(fromAirDropSignature);
  } catch (err) {
    console.log(err);
  }
};

const mainFunction = async () => {
  //   await getWalletBalance();
  await airDropSol();
  //   await getWalletBalance();
};

mainFunction();
