const assert = require('assert');
const anchor = require('@project-serum/anchor');
const {SystemProgram} = anchor.web3;

describe('my calculator dapp', () => {

  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);
  // anchor.setProvider(anchor.Provider.env());
  const calculator_keypair = anchor.web3.Keypair.generate();
  const calculator_program = anchor.workspace.Calculatordapp;

  // console.log('\n\n\ncalculator_program: ------------------->', calculator_program, '\n\n\n');
  console.log('\n\n\ncalculator_keypair.publicKey: ------------------->', calculator_keypair.publicKey.toBase58(), '\n');
  console.log('provider.wallet.publicKey: ------------------->', provider.wallet.publicKey.toBase58(), '\n\n');  
  // console.log('\n\n\nSystemProgram.programId: ------------------->', SystemProgram.programId, '\n\n\n');
});
