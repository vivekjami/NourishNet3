use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::generated::state::{
	AccountPDA,
	Donor,
	FoodRequest,
};
use crate::generated::errors::Nourishnet3Error;

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
pub fn finalize_food_request(
	program_id: &Pubkey,
	delegate: &AccountInfo,
	food_request: &mut AccountPDA<FoodRequest>,
	donor: &mut AccountPDA<Donor>,
	recipient: &AccountInfo,
) -> ProgramResult {
    // Implement your business logic here...
    // Check if the donor has enough total_donations to donate
    if donor.data.total_donations.saturating_sub(food_request.data.amount_requested as u128) <= 0 {
        return Err(Nourishnet3Error::InvalidInstruction.into());
    }

    // Check if the delegate is the authorized user by the donor
    if donor.data.delegate != *delegate.key {
        return Err(Nourishnet3Error::InvalidInstruction.into());
    }

    // Check if the recipient to which we will transfer the funds is the recipient that requested the food_request
    if *recipient.key != food_request.data.recipient {
        return Err(Nourishnet3Error::InvalidInstruction.into());
    }

    // Update the donor state
    donor.data.total_donations -= food_request.data.amount_requested as u128;
    donor.data.active_donations += food_request.data.amount_requested as u128;
    food_request.data.approval_status = true;

    // Transfer from donor account to recipient's account
    **donor.info.try_borrow_mut_lamports()? -= food_request.data.amount_requested;
    **recipient.try_borrow_mut_lamports()? += food_request.data.amount_requested;


    Ok(())
}