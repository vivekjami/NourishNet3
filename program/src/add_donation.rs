use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::generated::state::{
	AccountPDA,
	Donor,
};


/// Adds a new donation to the donor account.
///
/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] 
/// 1. `[writable, signer]` delegate: [AccountInfo] 
/// 2. `[writable]` donor: [Donor] 
///
/// Data:
/// - amount: [u64] The amount to be added to the total donations.
pub fn add_donation(
	program_id: &Pubkey,
	delegate: &AccountInfo,
	donor: &mut AccountPDA<Donor>,
	donation_amount: u64,
) -> ProgramResult {
    // Implement your business logic here...
    // Update donor's total_donations amount
       donor.data.total_donations += donation_amount as u128;

       // Transfer from delegate account to donor's account
       **delegate.try_borrow_mut_lamports()? -= donation_amount;
       **donor.info.try_borrow_mut_lamports()? += donation_amount;

    Ok(())
}