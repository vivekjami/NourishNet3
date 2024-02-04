// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use crate::generated::errors::Nourishnet3Error;

#[derive(BorshSerialize, Debug)]
pub enum Nourishnet3Instruction {
/// After deploying the contract, this must be the first instruction
/// to call. It will configure the donor account.
///
/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] 
/// 1. `[writable, signer]` delegate: [AccountInfo] This will be the account that has permission to update the donor and approve requests.
/// 2. `[writable]` donor: [Donor] 
/// 3. `[]` system_program: [AccountInfo] Auto-generated, for account initialization
///
/// Data:
/// - donation_amount: [u64] The fee percentage for processing donations.
/// - fee: [u8] 
	CreateDonor(CreateDonorArgs),

/// Adds a new donation to the donor account.
///
/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] 
/// 1. `[writable, signer]` delegate: [AccountInfo] 
/// 2. `[writable]` donor: [Donor] 
///
/// Data:
/// - donation_amount: [u64] The amount to be added to the total donations.
	AddDonation(AddDonationArgs),

/// A recipient can use this method to request food aid.
///
/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] 
/// 1. `[signer]` recipient: [AccountInfo] 
/// 2. `[writable]` food_request: [FoodRequest] 
/// 3. `[]` donor: [Donor] 
/// 4. `[]` system_program: [AccountInfo] Auto-generated, for account initialization
///
/// Data:
/// - amount_requested: [u64] The requested amount of food aid.
/// - needs_verification_url: [String] URL to the recipient's verification document.
/// - food_request_seed_index: [u32] Auto-generated, from input food_request of type [FoodRequest] set the seed named index, required by the type
	RequestFoodAid(RequestFoodAidArgs),

/// Finalize the food aid request and update the FoodRequest account.
///
/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] 
/// 1. `[writable, signer]` delegate: [AccountInfo] 
/// 2. `[writable]` food_request: [FoodRequest] 
/// 3. `[writable]` donor: [Donor] 
/// 4. `[writable]` recipient: [AccountInfo] 
///
/// Data:
/// - food_request_seed_index: [u32] Auto-generated, from input food_request of type [FoodRequest] set the seed named index, required by the type
	FinalizeFoodRequest(FinalizeFoodRequestArgs),

/// Update donor information, including reward points and active donations.
///
/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] 
/// 1. `[writable, signer]` delegate: [AccountInfo] 
/// 2. `[writable]` donor: [Donor] 
///
/// Data:
/// - reward_points: [u128] Updated reward points for the donor.
/// - active_donations: [u128] Updated active donation amount for the donor.
	UpdateDonorInfo(UpdateDonorInfoArgs),

}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CreateDonorArgs {
	pub donation_amount: u64,
	pub fee: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct AddDonationArgs {
	pub donation_amount: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct RequestFoodAidArgs {
	pub amount_requested: u64,
	pub needs_verification_url: String,
	pub food_request_seed_index: u32,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct FinalizeFoodRequestArgs {
	pub food_request_seed_index: u32,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct UpdateDonorInfoArgs {
	pub reward_points: u128,
	pub active_donations: u128,
}

impl Nourishnet3Instruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input.split_first().ok_or(Nourishnet3Error::InvalidInstruction)?;

        Ok(match variant {
			0 => Self::CreateDonor(CreateDonorArgs::try_from_slice(rest).unwrap()),
			1 => Self::AddDonation(AddDonationArgs::try_from_slice(rest).unwrap()),
			2 => Self::RequestFoodAid(RequestFoodAidArgs::try_from_slice(rest).unwrap()),
			3 => Self::FinalizeFoodRequest(FinalizeFoodRequestArgs::try_from_slice(rest).unwrap()),
			4 => Self::UpdateDonorInfo(UpdateDonorInfoArgs::try_from_slice(rest).unwrap()),
			_ => return Err(Nourishnet3Error::InvalidInstruction.into())
        })
    }
}