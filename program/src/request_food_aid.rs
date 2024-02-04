use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::generated::state::{
	AccountPDA,
	Donor,
	FoodRequest,
};
use crate::generated::errors::Nourishnet3Error;

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
pub fn request_food_aid(
	program_id: &Pubkey,
	recipient: &AccountInfo,
	food_request: &mut AccountPDA<FoodRequest>,
	donor: &AccountPDA<Donor>,
	amount_requested: u64,
	needs_verification_url: String,
) -> ProgramResult {
    // Implement your business logic here...
    // Check if the donor has enough capital to donate
    if donor.data.total_donations.saturating_sub(amount_requested as u128) <= 0 {
        return Err(Nourishnet3Error::InvalidInstruction.into());
    }

    food_request.data.recipient = *recipient.key;
    food_request.data.amount_requested = amount_requested;
    food_request.data.amount_received = 0;
    food_request.data.needs_verification_url = needs_verification_url;
    food_request.data.approval_status = false;

    Ok(())
}