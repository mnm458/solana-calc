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

    
});
