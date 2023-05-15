import { describe, it } from 'mocha'
import { PublicKey } from '@solana/web3.js'
import { CONNECTION, PAYER } from './util/const'
import tableConfig from './util/lookup-table.json'

/**
 * Test the Arbitrage Program
 */
describe('Arbitrage Bot', async () => {
    const connection = CONNECTION
    const payer = PAYER
    const lookupTable = new PublicKey(tableConfig.table)
    it('Look for arbitrage opportunities', async () => {
        console.log('Testing Arbitrage Program')
        console.log(`   Address:    ${lookupTable.toBase58()}`)
    })
})
