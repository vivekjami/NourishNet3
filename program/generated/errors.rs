// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.

use num_derive::FromPrimitive;
use solana_program::decode_error::DecodeError;
use solana_program::msg;
use solana_program::program_error::{PrintProgramError, ProgramError};
use thiserror::Error;

#[derive(Error, FromPrimitive, Debug, Clone)]
pub enum Nourishnet3Error {
    #[error("Invalid Instruction")]
    InvalidInstruction,

    #[error("Invalid Signer Permission")]
    InvalidSignerPermission,

    #[error("Not The Expected Account Address")]
    NotExpectedAddress,

    #[error("Wrong Account Owner")]
    WrongAccountOwner,

    #[error("Invalid Account Len")]
    InvalidAccountLen,

    #[error("Executable Account Expected")]
    ExecutableAccountExpected,

 
}

impl From<Nourishnet3Error> for ProgramError {
    fn from(e: Nourishnet3Error) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for Nourishnet3Error {
    fn type_of() -> &'static str {
        "Nourishnet3Error"
    }
}

impl PrintProgramError for Nourishnet3Error {
    fn print<E>(&self)
    where
        E: 'static
            + std::error::Error
            + DecodeError<E>
            + PrintProgramError
            + num_traits::FromPrimitive,
    {
        match self {
            Nourishnet3Error::InvalidInstruction => msg!("Error: Invalid instruction"),
            Nourishnet3Error::InvalidSignerPermission => msg!("Error: The account is_signer value is not the expected one"),
            Nourishnet3Error::NotExpectedAddress => {
                msg!("Error: Not the expected account address")
            }
            Nourishnet3Error::WrongAccountOwner => msg!("Error: Wrong account owner"),
            Nourishnet3Error::InvalidAccountLen => msg!("Error: Invalid account length"),
            Nourishnet3Error::ExecutableAccountExpected => msg!("Error: Executable account expected"),
 
        }
    }
}