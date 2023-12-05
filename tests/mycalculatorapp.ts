const assert = require('assert')
const anchor = require('@coral-xyz/anchor')
const {SystemProgram} = anchor.web3

describe('mycalculatordapp', () => {
    const provider = anchor.getProvider();
    anchor.setProvider(provider);

    const calculator = anchor.web3.Keypair.generate()
    const program = anchor.workspace.mycalculatorapp 

    it("Create a calclutor", async() =>{
        await program.rpc.create("Hello World", {
            accounts: {
                calculator: calculator.publicKey,
                user: provider.wallet.publicKey,
                systemProgram: SystemProgram.programId
            },
            signers: [calculator]
        })
        const account = await program.account.calculator.fetch(calculator.publicKey)
        assert.ok(account.greeting === "Hello World")
    })

    it("Addition works", async() => {
        await program.rpc.add(new anchor.BN(2) , new anchor.BN(7),{
            accounts:{
                calculator: calculator.publicKey
            }
        })
        const account = await program.account.calculator.fetch(calculator.publicKey)
        assert.ok(account.result.eq(new anchor.BN(9)))
    })

    it("subtraction works" , async() => { 
        await program.rpc.sub(new anchor.BN(5), new anchor.BN(2) , {
            accounts: {
                calculator: calculator.publicKey
            }
        })
        const account = await program.account.calculator.fetch(calculator.publicKey)
        assert.ok(account.result.eq(new anchor.BN(3)))
    })

    it("Division works" , async() => { 
        await program.rpc.div(new anchor.BN(4), new anchor.BN(2) , {
            accounts: {
                calculator: calculator.publicKey
            }
        })
        const account = await program.account.calculator.fetch(calculator.publicKey)
        assert.ok(account.result.eq(new anchor.BN(2)))
        assert.ok(account.remainder.eq(new anchor.BN(0)))
    })

    it("Multiplication works" , async() => { 
        await program.rpc.mul(new anchor.BN(2), new anchor.BN(2) , {
            accounts: {
                calculator: calculator.publicKey
            }
        })
        const account = await program.account.calculator.fetch(calculator.publicKey)
        assert.ok(account.result.eq(new anchor.BN(4)))
    })
})  