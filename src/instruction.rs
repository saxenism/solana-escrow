// instruction.rs is responsible for decoding the instruction_data
use std::convert::TryInto;
use solana_program::program_error::ProgramError;

use crate::error::EscrowError::InvalidInstruction;

pub enum EscrowInstruction {
    /*
    Expected Flow:
    We have to start the trade by creating and populating an escrow account and transferring ownership of the given 
    temp token account to the PDA (Program derived address). PDAs are just addresses for programs. Don't think much about it.

    Accounts expected:
    0. `[signer]` The account of the person initializing the escrow
    1. `[writable]` Temporary token account that should be created prior to this instruction and owned by the initializer
    2. `[]` Initializer's token account for the token they will recieve should the trade go through
    3. `[writable]`: The account that will hold all necessary info about the trade
    4. `[]` The rent sysvar
    5. `[]` The token program
    */
    InitEscrow {
        /*
        This amount is not provided through an account but rather through the instruction_data
        This is the amount partyA expects to recieve of token Y
        */
        amount: u64
    }
}

impl EscrowInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        //We look at the first byte (called tag) and then determine how to decode
        // the rest if the data (rest) using match
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => Self::InitEscrow {
                amount: Self::unpack_amount(rest)?,
            },
            _ => return Err(InvalidInstruction.into()),
        })
    }

    fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
        let amount = input.get(..8)
                          .and_then(|slice| slice.try_into().ok())
                          .map(u64::from_le_bytes)
                          .ok_or(InvalidInstruction)?;
        Ok(amount)
    }
}