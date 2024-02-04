use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::generated::state::{
	AccountPDA,
	Donor,
};


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
pub fn create_donor(
	program_id: &Pubkey,
	delegate: &AccountInfo,
	donor: &mut AccountPDA<Donor>,
	donation_amount: u64,
	fee: u8,
) -> ProgramResult {
    // Implement your business logic here...
    donor.data.delegate = *delegate.key;
    donor.data.total_donations = donation_amount as u128;
    donor.data.active_donations = 0;
    donor.data.reward_points = 0;
    donor.data.fee = fee;
   
    // Transfer from delegate account to donor's account
    **delegate.try_borrow_mut_lamports()? -= donation_amount;
    **donor.info.try_borrow_mut_lamports()? += donation_amount;

    Ok(())
}