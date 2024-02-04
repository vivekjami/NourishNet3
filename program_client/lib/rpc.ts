// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.

import * as pda from "./pda";
import * as T from "./types";
import {
    Commitment,
    Connection,
    GetAccountInfoConfig,
    Keypair,
    PublicKey,
    sendAndConfirmTransaction,
    SystemProgram,
    Transaction,
    TransactionInstruction,
    TransactionSignature,
} from "@solana/web3.js";
import {deserialize, serialize} from "borsh";


let _programId: PublicKey;
let _connection: Connection;

export const initializeClient = (
    programId: PublicKey,
    connection: Connection
) => {
    _programId = programId;
    _connection = connection;
};

export enum Nourishnet3Instruction {
/**
 * After deploying the contract, this must be the first instruction
 * to call. It will configure the donor account.
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[writable, signer]` delegate: {@link PublicKey} This will be the account that has permission to update the donor and approve requests.
 * 2. `[writable]` donor: {@link Donor} 
 * 3. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - donation_amount: {@link BigInt} The fee percentage for processing donations.
 * - fee: {@link number} 
 */
    CreateDonor = 0,

/**
 * Adds a new donation to the donor account.
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[writable, signer]` delegate: {@link PublicKey} 
 * 2. `[writable]` donor: {@link Donor} 
 *
 * Data:
 * - donation_amount: {@link BigInt} The amount to be added to the total donations.
 */
    AddDonation = 1,

/**
 * A recipient can use this method to request food aid.
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[signer]` recipient: {@link PublicKey} 
 * 2. `[writable]` food_request: {@link FoodRequest} 
 * 3. `[]` donor: {@link Donor} 
 * 4. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - amount_requested: {@link BigInt} The requested amount of food aid.
 * - needs_verification_url: {@link string} URL to the recipient's verification document.
 * - food_request_seed_index: {@link number} Auto-generated, from input food_request of type [FoodRequest] set the seed named index, required by the type
 */
    RequestFoodAid = 2,

/**
 * Finalize the food aid request and update the FoodRequest account.
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[writable, signer]` delegate: {@link PublicKey} 
 * 2. `[writable]` food_request: {@link FoodRequest} 
 * 3. `[writable]` donor: {@link Donor} 
 * 4. `[writable]` recipient: {@link PublicKey} 
 *
 * Data:
 * - food_request_seed_index: {@link number} Auto-generated, from input food_request of type [FoodRequest] set the seed named index, required by the type
 */
    FinalizeFoodRequest = 3,

/**
 * Update donor information, including reward points and active donations.
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[writable, signer]` delegate: {@link PublicKey} 
 * 2. `[writable]` donor: {@link Donor} 
 *
 * Data:
 * - reward_points: {@link BigInt} Updated reward points for the donor.
 * - active_donations: {@link BigInt} Updated active donation amount for the donor.
 */
    UpdateDonorInfo = 4,
}

export type CreateDonorArgs = {
    feePayer: PublicKey;
    delegate: PublicKey;
    donationAmount: bigint;
    fee: number;
};


/**
 * ### Returns a {@link TransactionInstruction}
 * After deploying the contract, this must be the first instruction
 * to call. It will configure the donor account.
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[writable, signer]` delegate: {@link PublicKey} This will be the account that has permission to update the donor and approve requests.
 * 2. `[writable]` donor: {@link Donor} 
 * 3. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - donation_amount: {@link BigInt} The fee percentage for processing donations.
 * - fee: {@link number} 
 */
export const createDonor = (args: CreateDonorArgs): TransactionInstruction => {
    const data = serialize(
        {
            struct: {
                id: "u8",
                donation_amount: "u64",
                fee: "u8",
            },
        },
        {
            id: Nourishnet3Instruction.CreateDonor,
            donation_amount: args.donationAmount,
            fee: args.fee,
        }
    );

    const [donorPubkey] = pda.deriveDonorPDA(_programId);

    return new TransactionInstruction({
        data: Buffer.from(data),
        keys: [
            {pubkey: args.feePayer, isSigner: true, isWritable: true},
            {pubkey: args.delegate, isSigner: true, isWritable: true},
            {pubkey: donorPubkey, isSigner: false, isWritable: true},
            {pubkey: new PublicKey("11111111111111111111111111111111"), isSigner: false, isWritable: false},
        ],
        programId: _programId,
    });
};

/**
 * ### Returns a {@link TransactionSignature}
 * After deploying the contract, this must be the first instruction
 * to call. It will configure the donor account.
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[writable, signer]` delegate: {@link PublicKey} This will be the account that has permission to update the donor and approve requests.
 * 2. `[writable]` donor: {@link Donor} 
 * 3. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - donation_amount: {@link BigInt} The fee percentage for processing donations.
 * - fee: {@link number} 
 */
export const createDonorSendAndConfirm = async (
    args: Omit<CreateDonorArgs, "feePayer" |"delegate"> & { 
        signers: { feePayer: Keypair,  delegate: Keypair, }
 }
): Promise<TransactionSignature> => {
    const trx = new Transaction();


    trx.add(createDonor({
        ...args,
        feePayer: args.signers.feePayer.publicKey,
        delegate: args.signers.delegate.publicKey,
    }));

    return await sendAndConfirmTransaction(
        _connection,
        trx,
        [args.signers.feePayer, args.signers.delegate, ]
    );
};

export type AddDonationArgs = {
    feePayer: PublicKey;
    delegate: PublicKey;
    donationAmount: bigint;
};


/**
 * ### Returns a {@link TransactionInstruction}
 * Adds a new donation to the donor account.
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[writable, signer]` delegate: {@link PublicKey} 
 * 2. `[writable]` donor: {@link Donor} 
 *
 * Data:
 * - donation_amount: {@link BigInt} The amount to be added to the total donations.
 */
export const addDonation = (args: AddDonationArgs): TransactionInstruction => {
    const data = serialize(
        {
            struct: {
                id: "u8",
                donation_amount: "u64",
            },
        },
        {
            id: Nourishnet3Instruction.AddDonation,
            donation_amount: args.donationAmount,
        }
    );

    const [donorPubkey] = pda.deriveDonorPDA(_programId);

    return new TransactionInstruction({
        data: Buffer.from(data),
        keys: [
            {pubkey: args.feePayer, isSigner: true, isWritable: true},
            {pubkey: args.delegate, isSigner: true, isWritable: true},
            {pubkey: donorPubkey, isSigner: false, isWritable: true},
        ],
        programId: _programId,
    });
};

/**
 * ### Returns a {@link TransactionSignature}
 * Adds a new donation to the donor account.
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[writable, signer]` delegate: {@link PublicKey} 
 * 2. `[writable]` donor: {@link Donor} 
 *
 * Data:
 * - donation_amount: {@link BigInt} The amount to be added to the total donations.
 */
export const addDonationSendAndConfirm = async (
    args: Omit<AddDonationArgs, "feePayer" |"delegate"> & { 
        signers: { feePayer: Keypair,  delegate: Keypair, }
 }
): Promise<TransactionSignature> => {
    const trx = new Transaction();


    trx.add(addDonation({
        ...args,
        feePayer: args.signers.feePayer.publicKey,
        delegate: args.signers.delegate.publicKey,
    }));

    return await sendAndConfirmTransaction(
        _connection,
        trx,
        [args.signers.feePayer, args.signers.delegate, ]
    );
};

export type RequestFoodAidArgs = {
    feePayer: PublicKey;
    recipient: PublicKey;
    amountRequested: bigint;
    needsVerificationUrl: string;
    foodRequestSeedIndex: number;
};


/**
 * ### Returns a {@link TransactionInstruction}
 * A recipient can use this method to request food aid.
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[signer]` recipient: {@link PublicKey} 
 * 2. `[writable]` food_request: {@link FoodRequest} 
 * 3. `[]` donor: {@link Donor} 
 * 4. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - amount_requested: {@link BigInt} The requested amount of food aid.
 * - needs_verification_url: {@link string} URL to the recipient's verification document.
 * - food_request_seed_index: {@link number} Auto-generated, from input food_request of type [FoodRequest] set the seed named index, required by the type
 */
export const requestFoodAid = (args: RequestFoodAidArgs): TransactionInstruction => {
    const data = serialize(
        {
            struct: {
                id: "u8",
                amount_requested: "u64",
                needs_verification_url: "string",
                food_request_seed_index: "u32",
            },
        },
        {
            id: Nourishnet3Instruction.RequestFoodAid,
            amount_requested: args.amountRequested,
            needs_verification_url: args.needsVerificationUrl,
            food_request_seed_index: args.foodRequestSeedIndex,
        }
    );

    const [foodRequestPubkey] = pda.deriveFoodRequestPDA({
        recipient: args.recipient,
        index: args.foodRequestSeedIndex,
    }, _programId);
    const [donorPubkey] = pda.deriveDonorPDA(_programId);

    return new TransactionInstruction({
        data: Buffer.from(data),
        keys: [
            {pubkey: args.feePayer, isSigner: true, isWritable: true},
            {pubkey: args.recipient, isSigner: true, isWritable: false},
            {pubkey: foodRequestPubkey, isSigner: false, isWritable: true},
            {pubkey: donorPubkey, isSigner: false, isWritable: false},
            {pubkey: new PublicKey("11111111111111111111111111111111"), isSigner: false, isWritable: false},
        ],
        programId: _programId,
    });
};

/**
 * ### Returns a {@link TransactionSignature}
 * A recipient can use this method to request food aid.
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[signer]` recipient: {@link PublicKey} 
 * 2. `[writable]` food_request: {@link FoodRequest} 
 * 3. `[]` donor: {@link Donor} 
 * 4. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - amount_requested: {@link BigInt} The requested amount of food aid.
 * - needs_verification_url: {@link string} URL to the recipient's verification document.
 * - food_request_seed_index: {@link number} Auto-generated, from input food_request of type [FoodRequest] set the seed named index, required by the type
 */
export const requestFoodAidSendAndConfirm = async (
    args: Omit<RequestFoodAidArgs, "feePayer" |"recipient"> & { 
        signers: { feePayer: Keypair,  recipient: Keypair, }
 }
): Promise<TransactionSignature> => {
    const trx = new Transaction();


    trx.add(requestFoodAid({
        ...args,
        feePayer: args.signers.feePayer.publicKey,
        recipient: args.signers.recipient.publicKey,
    }));

    return await sendAndConfirmTransaction(
        _connection,
        trx,
        [args.signers.feePayer, args.signers.recipient, ]
    );
};

export type FinalizeFoodRequestArgs = {
    feePayer: PublicKey;
    delegate: PublicKey;
    recipient: PublicKey;
    foodRequestSeedIndex: number;
};


/**
 * ### Returns a {@link TransactionInstruction}
 * Finalize the food aid request and update the FoodRequest account.
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[writable, signer]` delegate: {@link PublicKey} 
 * 2. `[writable]` food_request: {@link FoodRequest} 
 * 3. `[writable]` donor: {@link Donor} 
 * 4. `[writable]` recipient: {@link PublicKey} 
 *
 * Data:
 * - food_request_seed_index: {@link number} Auto-generated, from input food_request of type [FoodRequest] set the seed named index, required by the type
 */
export const finalizeFoodRequest = (args: FinalizeFoodRequestArgs): TransactionInstruction => {
    const data = serialize(
        {
            struct: {
                id: "u8",
                food_request_seed_index: "u32",
            },
        },
        {
            id: Nourishnet3Instruction.FinalizeFoodRequest,
            food_request_seed_index: args.foodRequestSeedIndex,
        }
    );

    const [foodRequestPubkey] = pda.deriveFoodRequestPDA({
        recipient: args.recipient,
        index: args.foodRequestSeedIndex,
    }, _programId);
    const [donorPubkey] = pda.deriveDonorPDA(_programId);

    return new TransactionInstruction({
        data: Buffer.from(data),
        keys: [
            {pubkey: args.feePayer, isSigner: true, isWritable: true},
            {pubkey: args.delegate, isSigner: true, isWritable: true},
            {pubkey: foodRequestPubkey, isSigner: false, isWritable: true},
            {pubkey: donorPubkey, isSigner: false, isWritable: true},
            {pubkey: args.recipient, isSigner: false, isWritable: true},
        ],
        programId: _programId,
    });
};

/**
 * ### Returns a {@link TransactionSignature}
 * Finalize the food aid request and update the FoodRequest account.
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[writable, signer]` delegate: {@link PublicKey} 
 * 2. `[writable]` food_request: {@link FoodRequest} 
 * 3. `[writable]` donor: {@link Donor} 
 * 4. `[writable]` recipient: {@link PublicKey} 
 *
 * Data:
 * - food_request_seed_index: {@link number} Auto-generated, from input food_request of type [FoodRequest] set the seed named index, required by the type
 */
export const finalizeFoodRequestSendAndConfirm = async (
    args: Omit<FinalizeFoodRequestArgs, "feePayer" |"delegate"> & { 
        signers: { feePayer: Keypair,  delegate: Keypair, }
 }
): Promise<TransactionSignature> => {
    const trx = new Transaction();


    trx.add(finalizeFoodRequest({
        ...args,
        feePayer: args.signers.feePayer.publicKey,
        delegate: args.signers.delegate.publicKey,
    }));

    return await sendAndConfirmTransaction(
        _connection,
        trx,
        [args.signers.feePayer, args.signers.delegate, ]
    );
};

export type UpdateDonorInfoArgs = {
    feePayer: PublicKey;
    delegate: PublicKey;
    rewardPoints: bigint;
    activeDonations: bigint;
};


/**
 * ### Returns a {@link TransactionInstruction}
 * Update donor information, including reward points and active donations.
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[writable, signer]` delegate: {@link PublicKey} 
 * 2. `[writable]` donor: {@link Donor} 
 *
 * Data:
 * - reward_points: {@link BigInt} Updated reward points for the donor.
 * - active_donations: {@link BigInt} Updated active donation amount for the donor.
 */
export const updateDonorInfo = (args: UpdateDonorInfoArgs): TransactionInstruction => {
    const data = serialize(
        {
            struct: {
                id: "u8",
                reward_points: "u128",
                active_donations: "u128",
            },
        },
        {
            id: Nourishnet3Instruction.UpdateDonorInfo,
            reward_points: args.rewardPoints,
            active_donations: args.activeDonations,
        }
    );

    const [donorPubkey] = pda.deriveDonorPDA(_programId);

    return new TransactionInstruction({
        data: Buffer.from(data),
        keys: [
            {pubkey: args.feePayer, isSigner: true, isWritable: true},
            {pubkey: args.delegate, isSigner: true, isWritable: true},
            {pubkey: donorPubkey, isSigner: false, isWritable: true},
        ],
        programId: _programId,
    });
};

/**
 * ### Returns a {@link TransactionSignature}
 * Update donor information, including reward points and active donations.
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[writable, signer]` delegate: {@link PublicKey} 
 * 2. `[writable]` donor: {@link Donor} 
 *
 * Data:
 * - reward_points: {@link BigInt} Updated reward points for the donor.
 * - active_donations: {@link BigInt} Updated active donation amount for the donor.
 */
export const updateDonorInfoSendAndConfirm = async (
    args: Omit<UpdateDonorInfoArgs, "feePayer" |"delegate"> & { 
        signers: { feePayer: Keypair,  delegate: Keypair, }
 }
): Promise<TransactionSignature> => {
    const trx = new Transaction();


    trx.add(updateDonorInfo({
        ...args,
        feePayer: args.signers.feePayer.publicKey,
        delegate: args.signers.delegate.publicKey,
    }));

    return await sendAndConfirmTransaction(
        _connection,
        trx,
        [args.signers.feePayer, args.signers.delegate, ]
    );
};

// Getters

export const getDonor = async (
    publicKey: PublicKey,
    commitmentOrConfig: Commitment | GetAccountInfoConfig | undefined = "processed"
): Promise<T.Donor | undefined> => {
    const buffer = await _connection.getAccountInfo(publicKey, commitmentOrConfig);

    if (!buffer) {
        return undefined
    }

    if (buffer.data.length <= 0) {
        return undefined
    }

    return T.decodeDonor(deserialize(T.DonorSchema, buffer.data) as Record<string, unknown>);
}

export const getFoodRequest = async (
    publicKey: PublicKey,
    commitmentOrConfig: Commitment | GetAccountInfoConfig | undefined = "processed"
): Promise<T.FoodRequest | undefined> => {
    const buffer = await _connection.getAccountInfo(publicKey, commitmentOrConfig);

    if (!buffer) {
        return undefined
    }

    if (buffer.data.length <= 0) {
        return undefined
    }

    return T.decodeFoodRequest(deserialize(T.FoodRequestSchema, buffer.data) as Record<string, unknown>);
}


// Websocket Events

