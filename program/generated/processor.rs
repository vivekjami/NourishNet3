// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.

use std::str::FromStr;
use borsh::BorshSerialize;
use solana_program::account_info::{AccountInfo, next_account_info, next_account_infos};
use solana_program::borsh0_10::try_from_slice_unchecked;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::{invoke, invoke_signed};
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::system_instruction::create_account;
use solana_program::{msg, system_program};
use solana_program::sysvar::Sysvar;
use solana_program::program_pack::Pack;
use crate::generated::errors::Nourishnet3Error;
use crate::generated::instructions::Nourishnet3Instruction;

use crate::generated::state::{
	Account,
	AccountPDA,
	Donor,
	FoodRequest,
};
use crate::src::*;

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        data: &[u8],
    ) -> ProgramResult {
        let instruction = Nourishnet3Instruction::unpack(data)?;

        match instruction {
			Nourishnet3Instruction::CreateDonor(args) => {
				msg!("Instruction: CreateDonor");
				Self::process_create_donor(
					program_id,
					accounts, 
					args.donation_amount,
					args.fee,
				)
			}
			Nourishnet3Instruction::AddDonation(args) => {
				msg!("Instruction: AddDonation");
				Self::process_add_donation(
					program_id,
					accounts, 
					args.donation_amount,
				)
			}
			Nourishnet3Instruction::RequestFoodAid(args) => {
				msg!("Instruction: RequestFoodAid");
				Self::process_request_food_aid(
					program_id,
					accounts, 
					args.amount_requested,
					args.needs_verification_url,
					args.food_request_seed_index,
				)
			}
			Nourishnet3Instruction::FinalizeFoodRequest(args) => {
				msg!("Instruction: FinalizeFoodRequest");
				Self::process_finalize_food_request(
					program_id,
					accounts, 
					args.food_request_seed_index,
				)
			}
			Nourishnet3Instruction::UpdateDonorInfo(args) => {
				msg!("Instruction: UpdateDonorInfo");
				Self::process_update_donor_info(
					program_id,
					accounts, 
					args.reward_points,
					args.active_donations,
				)
			}
        }
    }

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
	pub fn process_create_donor(
		program_id: &Pubkey,
		accounts: &[AccountInfo],
		donation_amount: u64,
		fee: u8,
	) -> ProgramResult {
		let account_info_iter = &mut accounts.iter();
		let fee_payer_info = next_account_info(account_info_iter)?;
		let delegate_info = next_account_info(account_info_iter)?;
		let donor_info = next_account_info(account_info_iter)?;
		let system_program_info = next_account_info(account_info_iter)?;

		// Derive PDAs
		let (donor_pubkey, donor_bump) = Pubkey::find_program_address(
			&[b"donor"],
			program_id,
		);

		// Security Checks
		if fee_payer_info.is_signer != true {
			return Err(Nourishnet3Error::InvalidSignerPermission.into());
		}

		if delegate_info.is_signer != true {
			return Err(Nourishnet3Error::InvalidSignerPermission.into());
		}

		if *donor_info.key != donor_pubkey {
			return Err(Nourishnet3Error::NotExpectedAddress.into());
		}

		if *system_program_info.key != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(Nourishnet3Error::NotExpectedAddress.into());
		}


		// Accounts Initializations
		let space = Donor::LEN;
		let rent = Rent::get()?;
		let rent_minimum_balance = rent.minimum_balance(space);

		invoke_signed(
			&create_account(
				&fee_payer_info.key,
				&donor_info.key,
				rent_minimum_balance,
				space as u64,
				program_id,
			),
			&[fee_payer_info.clone(), donor_info.clone()],
			&[&[b"donor", &[donor_bump]]],
		)?;


		// Security Checks
		if *fee_payer_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(Nourishnet3Error::WrongAccountOwner.into());
		}

		if donor_info.data_len() != Donor::LEN {
			return Err(Nourishnet3Error::InvalidAccountLen.into());
		}


		// Accounts Deserialization
		let donor = &mut AccountPDA::new(
			&donor_info,
			try_from_slice_unchecked::<Donor>(&donor_info.data.borrow()).unwrap(),
			donor_bump,
		);

		// Calling STUB
		create_donor::create_donor(
			program_id,
			delegate_info,
			donor,
			donation_amount,
			fee,
		)?;

		// Accounts Serialization
		donor.data.serialize(&mut &mut donor_info.data.borrow_mut()[..])?;
		
		Ok(())
	}

/// Adds a new donation to the donor account.
///
/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] 
/// 1. `[writable, signer]` delegate: [AccountInfo] 
/// 2. `[writable]` donor: [Donor] 
///
/// Data:
/// - donation_amount: [u64] The amount to be added to the total donations.
	pub fn process_add_donation(
		program_id: &Pubkey,
		accounts: &[AccountInfo],
		donation_amount: u64,
	) -> ProgramResult {
		let account_info_iter = &mut accounts.iter();
		let fee_payer_info = next_account_info(account_info_iter)?;
		let delegate_info = next_account_info(account_info_iter)?;
		let donor_info = next_account_info(account_info_iter)?;

		// Derive PDAs
		let (donor_pubkey, donor_bump) = Pubkey::find_program_address(
			&[b"donor"],
			program_id,
		);

		// Security Checks
		if fee_payer_info.is_signer != true {
			return Err(Nourishnet3Error::InvalidSignerPermission.into());
		}

		if delegate_info.is_signer != true {
			return Err(Nourishnet3Error::InvalidSignerPermission.into());
		}

		if *donor_info.key != donor_pubkey {
			return Err(Nourishnet3Error::NotExpectedAddress.into());
		}



		// Security Checks
		if *fee_payer_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(Nourishnet3Error::WrongAccountOwner.into());
		}

		if donor_info.data_len() != Donor::LEN {
			return Err(Nourishnet3Error::InvalidAccountLen.into());
		}


		// Accounts Deserialization
		let donor = &mut AccountPDA::new(
			&donor_info,
			try_from_slice_unchecked::<Donor>(&donor_info.data.borrow()).unwrap(),
			donor_bump,
		);

		// Calling STUB
		add_donation::add_donation(
			program_id,
			delegate_info,
			donor,
			donation_amount,
		)?;

		// Accounts Serialization
		donor.data.serialize(&mut &mut donor_info.data.borrow_mut()[..])?;
		
		Ok(())
	}

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
	pub fn process_request_food_aid(
		program_id: &Pubkey,
		accounts: &[AccountInfo],
		amount_requested: u64,
		needs_verification_url: String,
		food_request_seed_index: u32,
	) -> ProgramResult {
		let account_info_iter = &mut accounts.iter();
		let fee_payer_info = next_account_info(account_info_iter)?;
		let recipient_info = next_account_info(account_info_iter)?;
		let food_request_info = next_account_info(account_info_iter)?;
		let donor_info = next_account_info(account_info_iter)?;
		let system_program_info = next_account_info(account_info_iter)?;

		// Derive PDAs
		let (food_request_pubkey, food_request_bump) = Pubkey::find_program_address(
			&[b"food_request", recipient_info.key.as_ref(), food_request_seed_index.to_le_bytes().as_ref()],
			program_id,
		);
		let (donor_pubkey, donor_bump) = Pubkey::find_program_address(
			&[b"donor"],
			program_id,
		);

		// Security Checks
		if fee_payer_info.is_signer != true {
			return Err(Nourishnet3Error::InvalidSignerPermission.into());
		}

		if recipient_info.is_signer != true {
			return Err(Nourishnet3Error::InvalidSignerPermission.into());
		}

		if *food_request_info.key != food_request_pubkey {
			return Err(Nourishnet3Error::NotExpectedAddress.into());
		}

		if *donor_info.key != donor_pubkey {
			return Err(Nourishnet3Error::NotExpectedAddress.into());
		}

		if *system_program_info.key != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(Nourishnet3Error::NotExpectedAddress.into());
		}


		// Accounts Initializations
		let space = FoodRequest::LEN;
		let rent = Rent::get()?;
		let rent_minimum_balance = rent.minimum_balance(space);

		invoke_signed(
			&create_account(
				&fee_payer_info.key,
				&food_request_info.key,
				rent_minimum_balance,
				space as u64,
				program_id,
			),
			&[fee_payer_info.clone(), food_request_info.clone()],
			&[&[b"food_request", recipient_info.key.as_ref(), food_request_seed_index.to_le_bytes().as_ref(), &[food_request_bump]]],
		)?;


		// Security Checks
		if *fee_payer_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(Nourishnet3Error::WrongAccountOwner.into());
		}

		if food_request_info.data_len() != FoodRequest::LEN {
			return Err(Nourishnet3Error::InvalidAccountLen.into());
		}

		if donor_info.data_len() != Donor::LEN {
			return Err(Nourishnet3Error::InvalidAccountLen.into());
		}


		// Accounts Deserialization
		let food_request = &mut AccountPDA::new(
			&food_request_info,
			try_from_slice_unchecked::<FoodRequest>(&food_request_info.data.borrow()).unwrap(),
			food_request_bump,
		);
		let donor = AccountPDA::new(
			&donor_info,
			try_from_slice_unchecked::<Donor>(&donor_info.data.borrow()).unwrap(),
			donor_bump,
		);

		// Calling STUB
		request_food_aid::request_food_aid(
			program_id,
			recipient_info,
			food_request,
			&donor,
			amount_requested,
			needs_verification_url,
		)?;

		// Accounts Serialization
		food_request.data.serialize(&mut &mut food_request_info.data.borrow_mut()[..])?;
		
		Ok(())
	}

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
	pub fn process_finalize_food_request(
		program_id: &Pubkey,
		accounts: &[AccountInfo],
		food_request_seed_index: u32,
	) -> ProgramResult {
		let account_info_iter = &mut accounts.iter();
		let fee_payer_info = next_account_info(account_info_iter)?;
		let delegate_info = next_account_info(account_info_iter)?;
		let food_request_info = next_account_info(account_info_iter)?;
		let donor_info = next_account_info(account_info_iter)?;
		let recipient_info = next_account_info(account_info_iter)?;

		// Derive PDAs
		let (food_request_pubkey, food_request_bump) = Pubkey::find_program_address(
			&[b"food_request", recipient_info.key.as_ref(), food_request_seed_index.to_le_bytes().as_ref()],
			program_id,
		);
		let (donor_pubkey, donor_bump) = Pubkey::find_program_address(
			&[b"donor"],
			program_id,
		);

		// Security Checks
		if fee_payer_info.is_signer != true {
			return Err(Nourishnet3Error::InvalidSignerPermission.into());
		}

		if delegate_info.is_signer != true {
			return Err(Nourishnet3Error::InvalidSignerPermission.into());
		}

		if *food_request_info.key != food_request_pubkey {
			return Err(Nourishnet3Error::NotExpectedAddress.into());
		}

		if *donor_info.key != donor_pubkey {
			return Err(Nourishnet3Error::NotExpectedAddress.into());
		}



		// Security Checks
		if *fee_payer_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(Nourishnet3Error::WrongAccountOwner.into());
		}

		if food_request_info.data_len() != FoodRequest::LEN {
			return Err(Nourishnet3Error::InvalidAccountLen.into());
		}

		if donor_info.data_len() != Donor::LEN {
			return Err(Nourishnet3Error::InvalidAccountLen.into());
		}


		// Accounts Deserialization
		let food_request = &mut AccountPDA::new(
			&food_request_info,
			try_from_slice_unchecked::<FoodRequest>(&food_request_info.data.borrow()).unwrap(),
			food_request_bump,
		);
		let donor = &mut AccountPDA::new(
			&donor_info,
			try_from_slice_unchecked::<Donor>(&donor_info.data.borrow()).unwrap(),
			donor_bump,
		);

		// Calling STUB
		finalize_food_request::finalize_food_request(
			program_id,
			delegate_info,
			food_request,
			donor,
			recipient_info,
		)?;

		// Accounts Serialization
		food_request.data.serialize(&mut &mut food_request_info.data.borrow_mut()[..])?;		donor.data.serialize(&mut &mut donor_info.data.borrow_mut()[..])?;
		
		Ok(())
	}

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
	pub fn process_update_donor_info(
		program_id: &Pubkey,
		accounts: &[AccountInfo],
		reward_points: u128,
		active_donations: u128,
	) -> ProgramResult {
		let account_info_iter = &mut accounts.iter();
		let fee_payer_info = next_account_info(account_info_iter)?;
		let delegate_info = next_account_info(account_info_iter)?;
		let donor_info = next_account_info(account_info_iter)?;

		// Derive PDAs
		let (donor_pubkey, donor_bump) = Pubkey::find_program_address(
			&[b"donor"],
			program_id,
		);

		// Security Checks
		if fee_payer_info.is_signer != true {
			return Err(Nourishnet3Error::InvalidSignerPermission.into());
		}

		if delegate_info.is_signer != true {
			return Err(Nourishnet3Error::InvalidSignerPermission.into());
		}

		if *donor_info.key != donor_pubkey {
			return Err(Nourishnet3Error::NotExpectedAddress.into());
		}



		// Security Checks
		if *fee_payer_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(Nourishnet3Error::WrongAccountOwner.into());
		}

		if donor_info.data_len() != Donor::LEN {
			return Err(Nourishnet3Error::InvalidAccountLen.into());
		}


		// Accounts Deserialization
		let donor = &mut AccountPDA::new(
			&donor_info,
			try_from_slice_unchecked::<Donor>(&donor_info.data.borrow()).unwrap(),
			donor_bump,
		);

		// Calling STUB
		update_donor_info::update_donor_info(
			program_id,
			delegate_info,
			donor,
			reward_points,
			active_donations,
		)?;

		// Accounts Serialization
		donor.data.serialize(&mut &mut donor_info.data.borrow_mut()[..])?;
		
		Ok(())
	}
}