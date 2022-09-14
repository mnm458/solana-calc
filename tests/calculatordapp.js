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


  it('create mycalculator account', async () => {
    // Add your test here.
    const tx = await calculator_program.rpc.create("Create new calculator account..!", {
      accounts: { 
        calculator: calculator_keypair.publicKey, 
        user: provider.wallet.publicKey, 
        systemProgram: SystemProgram.programId
      },
      signers: [calculator_keypair] 
    });
    console.log("Create new calculator account..!", tx);
    const account = await calculator_program.account.calculator.fetch(calculator_keypair.publicKey);
    assert.ok(account.message === "Create new calculator account..!");
  });

  it('addition', async () => {
    const tx = await calculator_program.rpc.addition(new anchor.BN(5), new anchor.BN(6), {
      accounts: { 
        calculator: calculator_keypair.publicKey
      }
    });
    console.log("Create new calculator account..!", tx);
    const account = await calculator_program.account.calculator.fetch(calculator_keypair.publicKey);
    assert.ok(account.result.eq(new anchor.BN(11)));
  });

  it('subtraction', async () => {
    const tx = await calculator_program.rpc.subtraction(new anchor.BN(5), new anchor.BN(6), {
      accounts: { 
        calculator: calculator_keypair.publicKey
      }
    });
    console.log("Create new calculator account..!", tx);
    const account = await calculator_program.account.calculator.fetch(calculator_keypair.publicKey);
    assert.ok(account.result.eq(new anchor.BN(-1)));
  });

  it('multiplication', async () => {
    const tx = await calculator_program.rpc.multiplication(new anchor.BN(5), new anchor.BN(6), {
      accounts: { 
        calculator: calculator_keypair.publicKey
      }
    });
    console.log("Create new calculator account..!", tx);
    const account = await calculator_program.account.calculator.fetch(calculator_keypair.publicKey);
  
  });


});
