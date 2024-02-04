// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.

import {PublicKey} from "@solana/web3.js";

export const deriveDonorPDA = (programId: PublicKey): [PublicKey, number] => {
    return PublicKey.findProgramAddressSync(
        [
            Buffer.from("donor"),
        ],
        programId,
    )
};

export type FoodRequestSeeds = {
    recipient: PublicKey, 
    index: number, 
};

export const deriveFoodRequestPDA = (
    seeds: FoodRequestSeeds,
    programId: PublicKey
): [PublicKey, number] => {
    return PublicKey.findProgramAddressSync(
        [
            Buffer.from("food_request"),
            seeds.recipient.toBuffer(),
            Buffer.from(Uint32Array.from([seeds.index]).buffer),
        ],
        programId,
    )
};

