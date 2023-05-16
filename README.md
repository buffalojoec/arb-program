# ✨ Quest 5 - It's an Arbitrage Pirate's Life for Me

📘 A real scallywag always looks for the finest opportunities to make a quick buck!

When it comes to goods markets (swaps), there’s no better opportunity than an arbitrage opportunity.

We’re going to learn about how arbitrage works, how to build an arbitrage program, and how we can place arbitrage trades across other pirates’ markets!

---

### [<img src="https://raw.githubusercontent.com/solana-developers/pirate-bootcamp/main/docs/images/slides-icon.svg" alt="slides" width="20" align="center"/> Presentation Slides (coming soon)](https://github.com/buffalojoec/nyc-bootcamp-arb-program)

### Arbitrage

When we consider a Liquidity Pool in a Decentralized Exchange (DEX), we know that we will get less return for our swap the smaller the liquidity amount is for the asset we're requesting.  
In other words, for any asset we intend to pay `p`, the asset we will receive `r` will depend on the balance of `r` in the pool.  
Less of `r` in the pool = smaller number for `r`. More of `r` in the pool = larger number for `r`.  
The balance of `p` in the pool also matters, but we can assume a large discrepency in `r` values across two pools insinuates a different in balances of `p` as well.

If we take `p` and calculate `r` for two pools and see Pool #1 is offering a much lesser quantity than Pool #2, we can assume:

-   Pool #1 has less of `r` and more of `p` than Pool #2
-   Pool #2 has more of `r` and less of `p` than Pool #1
-   We can acquire `r` on Pool #2 and sell to Pool #1 for our original asset, thus generating arbitrage profit

### Our Bot

Our program (bot) is designed to evaluate all possible asset pairings for the provided accounts and determine if there is in fact an arbitrage opportunity amongst the combinations. If there is, it will place a trade between two swap pools.

One of the benefits of our bot's design is the use of **Simulated Transactions** to ensure **we are only paying for a transaction fee when we know for certain we have a profitable trade**.

This is possible because Solana is designed - by default - to simulate transactions before actually sending them to the network to catch errors. This is called a "preflight" check.

Because Solana makes use of "preflight" checks, we can simply write our arbitrage program to result in an error every time no arbitrage opportunity is present. This way, when our program's simluated "preflight" does _not_ result in an error, we know we have a profitable opportunity and thus can send the actual transaction.

The best part about this architecture is we don't even have to modify our client, only our program!

### To Build:

-   Arbitrage Bot UI
    -   Want to build a cool UI for interacting with the arb bot via connected wallet
        -   You should be able to see the assets you hold in your connected wallet
        -   You should be able to get a preview of what the swap is going to be (amount to recieve)
