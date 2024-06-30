// import {
//   establishConnection,
//   establishPayer,
//   checkProgram,
//   sayHello,
//   reportGreetings,
// } from "./hello_world";
const {
  establishConnection,
  establishPayer,
  checkProgram,
  sayHello,
  reportGreetings,
} = require("./hello_world");

async function main() {
  console.log("Let's say hello to a Solana account...");

  await establishConnection();

  await establishPayer();

  await checkProgram();

  await sayHello();

  await reportGreetings();

  console.log("Success");
}

main().then(
  () => process.exit(),
  (err) => {
    console.error(err);
    process.exit(-1);
  }
);
