const {
  Connection,
  PublicKey,
  Transaction,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
} = require("@solana/web3.js");
const { Program, Provider, web3 } = require("@project-serum/anchor");
const { clickcrateIDL } = require("./utils");

const PROGRAM_ID = "zLTwC7jeA9miBYVxzbTmDzEMXTobXQ3nxD8BY6M29nF";

const getProvider = () => {
  const connection = new Connection(
    "https://api.devnet.solana.com",
    "confirmed"
  );
  const wallet = web3.Keypair.generate(); // Replace with your own wallet
  return new Provider(connection, wallet, Provider.defaultOptions());
};

const getProgram = () => {
  const provider = getProvider();
  return new Program(clickcrateIDL, PROGRAM_ID, provider);
};

const createClickCrate = async (
  name,
  placementType,
  assetType,
  userGraphUri,
  saleFee
) => {
  const program = getProgram();
  const tx = await program.methods
    .createClickcrate(name, placementType, assetType, userGraphUri, saleFee)
    .accounts({
      clickcrate: (
        await PublicKey.findProgramAddress(
          [Buffer.from("clickcrate")],
          program.programId
        )
      )[0],
      payer: program.provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    })
    .rpc();

  console.log("ClickCrate created:", tx);
};

const registerClickCrate = async (clickcrateAddress) => {
  const program = getProgram();
  const tx = await program.methods
    .registerClickcrate()
    .accounts({
      clickcrate: new PublicKey(clickcrateAddress),
    })
    .rpc();

  console.log("ClickCrate registered:", tx);
};

const createProduct = async (
  name,
  placementType,
  assetType,
  userProfileUri,
  listingOriginId,
  listingAccessToken
) => {
  const program = getProgram();
  const tx = await program.methods
    .createProduct(
      name,
      placementType,
      assetType,
      userProfileUri,
      listingOriginId,
      listingAccessToken
    )
    .accounts({
      product: (
        await PublicKey.findProgramAddress(
          [Buffer.from("product")],
          program.programId
        )
      )[0],
      payer: program.provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    })
    .rpc();

  console.log("Product created:", tx);
};

const registerProduct = async (productAddress, clickcrateAddress) => {
  const program = getProgram();
  const tx = await program.methods
    .registerProduct()
    .accounts({
      product: new PublicKey(productAddress),
      clickcrate: new PublicKey(clickcrateAddress),
    })
    .rpc();

  console.log("Product registered:", tx);
};

const makePurchase = async (productAddress, buyerAddress, amountPaid) => {
  const program = getProgram();
  const tx = await program.methods
    .makePurchase()
    .accounts({
      purchase: (
        await PublicKey.findProgramAddress(
          [Buffer.from("purchase")],
          program.programId
        )
      )[0],
      product: new PublicKey(productAddress),
      buyer: new PublicKey(buyerAddress),
      systemProgram: SystemProgram.programId,
    })
    .rpc();

  console.log("Purchase made:", tx);
};

module.exports = {
  createClickCrate,
  registerClickCrate,
  createProduct,
  registerProduct,
  makePurchase,
};
