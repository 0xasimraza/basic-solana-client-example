use anchor_client::{
    solana_client::rpc_client::RpcClient,
    solana_sdk::{
        commitment_config::CommitmentConfig, native_token::LAMPORTS_PER_SOL, pubkey::Pubkey,
        signature::Keypair, signer::Signer, system_program,
    },
    Client, Cluster,
};
use counter_rs_test_example::{
    accounts,
    instruction::{Decrement, Increment, Initialize},
    Counter,
};
use std::rc::Rc;

fn main() -> anyhow::Result<()> {
    let connection = RpcClient::new_with_commitment(
        "http://127.0.0.1:8899", // Local validator URL
        CommitmentConfig::confirmed(),
    );

    // Generate Keypairs and request airdrop
    let payer = Keypair::new();
    let counter = Keypair::new();
    println!("Generated Keypairs:");
    println!("   Payer: {}", payer.pubkey());
    println!("   Counter: {}", counter.pubkey());

    println!("\nRequesting 1 SOL airdrop to payer");
    let airdrop_signature = connection.request_airdrop(&payer.pubkey(), LAMPORTS_PER_SOL)?;

    // Wait for airdrop confirmation
    while !connection.confirm_transaction(&airdrop_signature)? {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    println!("   Airdrop confirmed!");

    // Create program client
    let provider = Client::new_with_options(
        Cluster::Localnet,
        Rc::new(payer),
        CommitmentConfig::confirmed(),
    );
    let program_id = Pubkey::try_from("AQGaUitJF6dcuM4izL7QKTCfaNDsEFwFx2JoHYDQwd6p")?;
    let program = provider.program(program_id)?;

    // Build and send instructions
    println!("\nSend transaction with initialize and increment instructions");
    let initialize_ix = program
        .request()
        .accounts(accounts::Initialize {
            counter: counter.pubkey(),
            payer: program.payer(),
            system_program: system_program::ID,
        })
        .args(Initialize {})
        .instructions()?
        .remove(0);

    let increment_ix = program
        .request()
        .accounts(accounts::Increment {
            counter: counter.pubkey(),
        })
        .args(Increment {})
        .instructions()?
        .remove(0);

    let signature = program
        .request()
        .instruction(initialize_ix)
        .instruction(increment_ix)
        .signer(&counter)
        .send()?;
    println!("   Transaction confirmed: {}", signature);

    println!("\nFetch counter account data");
    let counter_account: Counter = program.account(counter.pubkey())?;
    println!("   Counter value: {}", counter_account.count);

    println!("\nSend transaction with decrement instruction");
    let decrement_ix = program
        .request()
        .accounts(accounts::Decrement {
            counter: counter.pubkey(),
        })
        .args(Decrement {})
        .instructions()?
        .remove(0);

    let signature = program.request().instruction(decrement_ix).send()?;
    println!("   Transaction confirmed: {}", signature);

    println!("\nFetch counter account data");
    let counter_account: Counter = program.account(counter.pubkey())?;
    println!("   Counter value: {}", counter_account.count);
    Ok(())
}
