import { describe, it } from 'mocha'
import fs from 'fs'
import { PublicKey } from '@solana/web3.js'
import assetsConfig from './util/assets.json'
import { CONNECTION, PAYER } from './util/const'
import {
    createAndPopulateAddressLookupTable,
    printAddressLookupTable,
} from './util/transaction'

/**
 * Test script to create an Address Lookup Table
 */
describe('Create an Address Lookup Table', async () => {
    const connection = CONNECTION
    const payer = PAYER
    const addresses = assetsConfig.assets.map((o) => new PublicKey(o.address))
    it('Create the Lookup Table', async () => {
        const [lookupTable, sx] = await createAndPopulateAddressLookupTable(
            connection,
            payer,
            addresses
        )
        console.log('Lookup Table created successfully.')
        console.log(`   Address:    ${lookupTable.toBase58()}`)
        console.log(`   Signature:  ${sx}`)
        printAddressLookupTable(connection, lookupTable)
        fs.writeFileSync(
            './tests/util/lookup-table.json',
            JSON.stringify({
                table: lookupTable.toBase58(),
            })
        )
    })
})
