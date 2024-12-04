#![allow(dead_code)]
use anyhow::Result;
use clap::Parser;
use solana_sdk::{
    program_pack::Pack
    ,
    signature::Signer
    ,
};
use std::str::FromStr;

mod instructions;
use instructions::events_instructions_parse::*;

#[derive(Debug, Parser)]
pub struct Opts {
    #[arg(long)]
    pub instr_data: String,
}

fn main() -> Result<()> {
    let opts = Opts::parse();
    handle_program_instruction(&opts.instr_data, InstructionDecodeType::Base58)?;

    Ok(())
}
