import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, sendAndConfirmTransaction, SystemProgram, Transaction } from "@solana/web3.js";
import * as fs from "fs/promises";
import * as path from "path";
import * as os from "os";
import {
    createDonorSendAndConfirm,
    requestFoodAidSendAndConfirm,
    finalizeFoodRequestSendAndConfirm,
    getFoodRequest,
    getDonor,
    requestFoodAid,
} from "./index";

async function main(feePayer: Keypair) {
    const connection = new Connection("http://127.0.0.1:8899", {
        commitment: "confirmed"
    });

    // TODO: Specify the smart contract Program Id we saved from when we deploy the smart contract
    const progId = new PublicKey("PASTE_YOUR_PROGRAM_ID_HERE");

    requestFoodAid(progId, connection);

    /**
    * Create wallets to represent the broker creator and 2 clients
    */
    const delegate = Keypair.generate();
    const food_request_1 = Keypair.generate();
    const food_request_2 = Keypair.generate();

    const rent = await connection.getMinimumBalanceForRentExemption(0);
    const total_donations = 5 * LAMPORTS_PER_SOL;


    await sendAndConfirmTransaction(
        connection,
        new Transaction()
            .add(
                SystemProgram.createAccount({
                    fromPubkey: feePayer.publicKey,
                    newAccountPubkey: delegate.publicKey,
                    space: 0,
                    lamports: rent + total_donations,
                    programId: progId,
                })
            )
            .add(
                SystemProgram.createAccount({
                    fromPubkey: feePayer.publicKey,
                    newAccountPubkey: food_request_1.publicKey,
                    space: 0,
                    lamports: rent ,
                    programId: progId,
                })
            )
            .add(
                SystemProgram.createAccount({
                    fromPubkey: feePayer.publicKey,
                    newAccountPubkey: food_request_2.publicKey,
                    space: 0,
                    lamports: rent ,
                    programId: progId,
                })
            ),
        [feePayer, delegate, food_request_1, food_request_2]
    );

    const [donorPK] = deriveDonorPDA(progId);
    let donor = await getDonor(donorPK)

    // Donor can only exists once per contract
    // so let's check if we haven't created it
    if (!donor) {
        await createDonorSendAndConfirm({
            amount: BigInt (total_donations),
            fee: 10,
            signers: {
                feePayer,
                delegate,
            }
        });
        donor = await getDonor(donorPK)
    } 

    // Log current state of the donor
    console.log("+=====+DONOR STATE+=====+")
    console.info(donor);

    // Request food 1 by user 1
    const [user1Request1] = deriveFoodRequestPDA({
        food_request: food_request_1.publicKey,
        index: 1,
    }, progId)
    await requestFoodAidSendAndConfirm({
        amount: BigInt(2.5 * LAMPORTS_PER_SOL),
        kycUrl: "https://example.com",
        foodRequestSeedIndex: 1,
        signers: {
            feePayer,
            food_request: food_request_1,
        }
    });
    console.log("+=====+USER ONE REQUEST ONE STATE+=====+")
    let user1Request1State = await getFoodRequest(user1Request1);
    console.info(user1Request1State);

    // Aprove/Finalize request 1 by user 1
    await finalizeFoodRequestSendAndConfirm({
        food_request: food_request_1.publicKey,
        foodRequestSeedIndex: 1,
        signers: {
            feePayer,
            delegate,
        }
    });
    console.log("+=====+USER ONE REQUEST ONE STATE+=====+")
    user1Request1State = await getFoodRequest(user1Request1);
    console.info(user1Request1State);

    console.log("+=====+DONOR STATE+=====+")
    donor = await getDonor(donorPK)
    console.info(donor);

    // Request food 2 by user 1
    const [user1Request2] = deriveFoodRequestPDA({
        food_request: food_request_1.publicKey,
        index: 2,
    }, progId)
    await requestFoodAidSendAndConfirm({
        amount: BigInt(2 * LAMPORTS_PER_SOL),
        kycUrl: "https://example.com",
        foodRequestSeedIndex: 2,
        signers: {
            feePayer,
            food_request: food_request_1,
        }
    });
    console.log("+=====+USER ONE REQUEST TWO STATE+=====+")
    let user1Request2State = await getFoodRequest(user1Request2);
    console.info(user1Request2State);

    // Aprove/finalize request 2 by user 1
    await finalizeFoodRequestSendAndConfirm({
        food_request : food_request_1.publicKey,
        foodRequestSeedIndex: 2,
        signers: {
            feePayer,
            delegate,
        }
    });
    console.log("+=====+USER ONE REQUEST TWO STATE+=====+")
    user1Request2State = await getFoodRequest(user1Request2);
    console.info(user1Requset2State);

    console.log("+=====+DONOR STATE+=====+")
    donor = await getDonor(donorPK)
    console.info(donor);

    // Request loan 1 by client 2
    // This request must fail because donor doesn't have capital
    try {
        const [user2Request1] = deriveFoodRequestPDA({
            food_request: food_request_2.publicKey,
            index: 1,
        }, progId)
        await requestFoodAidSendAndConfirm({
            amount: BigInt(2 * LAMPORTS_PER_SOL),
            kycUrl: "https://example.com",
            foodRequestSeedIndex: 1,
            signers: {
                feePayer,
                food_request : food_request_2,
            }
        });
        let user2Request1State = await getFoodRequest(user2Request1);
        console.info(user2Request1State);
    } catch (e) {
        console.info(`Donor doesn't have capital to donate: ${e}`)
    }

    
    console.log("+=====+DONOR STATE+=====+")
    donor = await getDonor(donorPK)
    console.info(donor);
}

// 5_000_000_000n
// 450_000_007n

fs.readFile(path.join(os.homedir(), ".config/solana/id.json"))
    .then(file => main(Keypair.fromSecretKey(new Uint8Array(JSON.parse(file.toString())))));