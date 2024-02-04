// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.

import type {Schema} from 'borsh';
import type {Decoded} from "./utils";
import {PublicKey} from "@solana/web3.js";
import { deserialize } from "borsh";

/// Donor is an account that can exist multiple times per contract. 
/// It stores all the required information to manage food donations.
export interface Donor {
  delegate: PublicKey;
  totalDonations: bigint;
  activeDonations: bigint;
  rewardPoints: bigint;
  fee: number;
}

export const decodeDonor = (decoded: Decoded): Donor => ({
    delegate: new PublicKey(decoded["delegate"] as Uint8Array),
    totalDonations: decoded["total_donations"] as bigint,
    activeDonations: decoded["active_donations"] as bigint,
    rewardPoints: decoded["reward_points"] as bigint,
    fee: decoded["fee"] as number,
});

export const DonorSchema: Schema =  {
    struct: {
        delegate: { array: { type: "u8", len: 32 } },
        total_donations: "u128",
        active_donations: "u128",
        reward_points: "u128",
        fee: "u8",
    }
};

/// FoodRequest is an account that will exist as many times as required
/// per recipient. It stores a request for food aid and if approved,
/// tracks the amount received.
export interface FoodRequest {
  recipient: PublicKey;
  amountRequested: bigint;
  amountReceived: bigint;
  needsVerificationUrl: string;
  approvalStatus: boolean;
}

export const decodeFoodRequest = (decoded: Decoded): FoodRequest => ({
    recipient: new PublicKey(decoded["recipient"] as Uint8Array),
    amountRequested: decoded["amount_requested"] as bigint,
    amountReceived: decoded["amount_received"] as bigint,
    needsVerificationUrl: decoded["needs_verification_url"] as string,
    approvalStatus: decoded["approval_status"] as boolean,
});

export const FoodRequestSchema: Schema =  {
    struct: {
        recipient: { array: { type: "u8", len: 32 } },
        amount_requested: "u64",
        amount_received: "u64",
        needs_verification_url: "string",
        approval_status: "bool",
    }
};



