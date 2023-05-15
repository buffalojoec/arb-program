import {
    Connection,
    Keypair,
    PublicKey,
    TransactionInstruction,
    VersionedTransaction,
    TransactionMessage,
    AddressLookupTableProgram,
} from '@solana/web3.js'
import { sleepSeconds } from '.'

/**
 *
 * Creates and populates an Address Lookup Table with the provided addresses
 *
 * @param connection Connection to Solana RPC
 * @param payer Transaction Fee Payer and Lookup Table Authority
 * @param addresses Addresses to put in the Lookup Table
 * @returns [Address of the Lookup Table, Transaction Signature]
 */
export async function createAndPopulateAddressLookupTable(
    connection: Connection,
    payer: Keypair,
    addresses: PublicKey[]
): Promise<[PublicKey, string]> {
    // You must use `max` for the derivation of the Lookup Table address to work consistently
    let recentSlot = await connection.getSlot('max')
    let [createLookupTableIx, lookupTable] =
        AddressLookupTableProgram.createLookupTable({
            authority: payer.publicKey,
            payer: payer.publicKey,
            recentSlot,
        })
    let extendLookupTableIx = AddressLookupTableProgram.extendLookupTable({
        addresses,
        authority: payer.publicKey,
        lookupTable,
        payer: payer.publicKey,
    })
    const sx = await sendTransactionV0(
        connection,
        [createLookupTableIx, extendLookupTableIx],
        payer
    )
    return [lookupTable, sx]
}

/**
 *
 * Print the contents of an Address Lookup Table
 *
 * @param connection Connection to Solana RPC
 * @param lookupTablePubkey The address of the Address Lookup Table
 */
export async function printAddressLookupTable(
    connection: Connection,
    lookupTablePubkey: PublicKey
): Promise<void> {
    await sleepSeconds(2)
    const lookupTableAccount = await connection
        .getAddressLookupTable(lookupTablePubkey)
        .then((res) => res.value)
    console.log(`Lookup Table: ${lookupTablePubkey}`)
    for (let i = 0; i < lookupTableAccount.state.addresses.length; i++) {
        const address = lookupTableAccount.state.addresses[i]
        console.log(`   Index: ${i}  Address: ${address.toBase58()}`)
    }
}

/**
 *
 * Builds and sends a transaction using the V0 format
 *
 * @param connection Connection to Solana RPC
 * @param instructions Instructions to send
 * @param payer Transaction Fee Payer
 * @returns The transaction signature
 */
export async function sendTransactionV0(
    connection: Connection,
    instructions: TransactionInstruction[],
    payer: Keypair
): Promise<string> {
    let blockhash = await connection
        .getLatestBlockhash()
        .then((res) => res.blockhash)
    const messageV0 = new TransactionMessage({
        payerKey: payer.publicKey,
        recentBlockhash: blockhash,
        instructions,
    }).compileToV0Message()
    const tx = new VersionedTransaction(messageV0)
    tx.sign([payer])
    return connection.sendTransaction(tx)
}

/**
 *
 * Builds and sends a transaction using the V0 format
 * using an Address Lookup Table
 *
 * @param connection Connection to Solana RPC
 * @param instructions Instructions to send
 * @param payer Transaction Fee Payer
 * @param lookupTablePubkey The address of the Address Lookup Table to use
 * @returns The transaction signature
 */
export async function sendTransactionV0WithLookupTable(
    connection: Connection,
    instructions: TransactionInstruction[],
    payer: Keypair,
    lookupTablePubkey: PublicKey
): Promise<string> {
    const lookupTableAccount = await connection
        .getAddressLookupTable(lookupTablePubkey)
        .then((res) => res.value)

    let blockhash = await connection
        .getLatestBlockhash()
        .then((res) => res.blockhash)

    const messageV0 = new TransactionMessage({
        payerKey: payer.publicKey,
        recentBlockhash: blockhash,
        instructions,
    }).compileToV0Message([lookupTableAccount])

    const tx = new VersionedTransaction(messageV0)
    tx.sign([payer])
    return connection.sendTransaction(tx)
}
