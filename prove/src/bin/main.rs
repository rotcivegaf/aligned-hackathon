use clap::Parser;
use sp1_sdk::{ProverClient, SP1Stdin};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

use bincode;

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const SOKOBAN_ELF: &[u8] = include_bytes!("../../../program/elf/riscv32im-succinct-zkvm-elf");

/// The arguments for the command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    execute: bool,

    #[clap(long)]
    prove: bool,

    #[clap(long)]
    output: String,
}

#[derive(Serialize, Deserialize)]
struct Output {
    inputs: String,
    score: u8,
    win: bool,
    end_frame: u16,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    if args.execute == args.prove {
        eprintln!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }

    println!("{:?}", args);

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();
    // moves is the serde output
    stdin.write(&args.output);
    // Setup the prover client.
    let client = ProverClient::new();

    if args.prove {
        // Setup the program for proving.
        let (pk, vk) = client.setup(SOKOBAN_ELF);

        // Generate the proof
        
        let proof = client
            .prove(&pk, stdin)
            .run()
            .expect("failed to generate proof");

        println!("Successfully generated proof!");

        // Verify the proof.
        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Successfully verified proof!");

        // Serialize the verification key and proof
        serde_json::to_string(&vk).expect("Failed to serialize verification key");
        serde_json::to_string(&proof).expect("Failed to serialize proof");

        // Get the raw bytes of the proof
        let proof = bincode::serialize(&proof).expect("Failed to serialize proof");


        // Save proof to file in raw byte format
        let mut file = File::create("proof.bin")?;
        file.write_all(&proof)?;
   
        println!("Proof saved to proof.bin");
    }

    Ok(())
}