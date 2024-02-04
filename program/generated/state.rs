// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::AccountInfo;
use solana_program::pubkey::Pubkey;

#[derive(Clone, Debug)]
pub struct Account<'a, 'b, T> {
    pub data: T,
    pub info: &'a AccountInfo<'b>,
}

#[derive(Clone, Debug)]
pub struct AccountPDA<'a, 'b, T> {
    pub data: T,
    pub info: &'a AccountInfo<'b>,
    pub bump: u8,
}

impl<'a, 'b, T> Account<'a, 'b, T> {
    pub fn new(info: &'a AccountInfo<'b>, data: T) -> Self {
        Self { data, info }
    }
}

impl<'a, 'b, T> AccountPDA<'a, 'b, T> {
    pub fn new(info: &'a AccountInfo<'b>, data: T, bump: u8) -> Self {
        Self { data, info, bump }
    }
}

/// Donor is an account that can exist multiple times per contract. 
/// It stores all the required information to manage food donations.
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Default)]
pub struct Donor {
	pub delegate: Pubkey,
	pub total_donations: u128,
	pub active_donations: u128,
	pub reward_points: u128,
	pub fee: u8,
}

impl Donor {
	pub const LEN: usize = 81; 
	}

/// FoodRequest is an account that will exist as many times as required
/// per recipient. It stores a request for food aid and if approved,
/// tracks the amount received.
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Default)]
pub struct FoodRequest {
	pub recipient: Pubkey,
	pub amount_requested: u64,
	pub amount_received: u64,
	pub needs_verification_url: String,
	pub approval_status: bool,
}

impl FoodRequest {
	pub const LEN: usize = 149; 
	}

