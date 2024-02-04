use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;
use solana_program::msg;

use crate::generated::state::{
	AccountPDA,
	Donor,
};


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
pub fn update_donor_info(
	program_id: &Pubkey,
	delegate: &AccountInfo,
	donor: &mut AccountPDA<Donor>,
	reward_points: u128,
	active_donations: u128,
) -> ProgramResult {
    // Implement your business logic here...
    // Update the broker state
    donor.data.reward_points += 1;

    msg!("Total Donated amount: {}, Reward_ponits: {}, Total_donations that can be done: {}", donor.data.active_donations, donor.data.reward_points, donor.data.total_donations);


    Ok(())
}