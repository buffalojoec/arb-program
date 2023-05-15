# NYC Bootcamp Arbitrage Program
Arbitrage program for NYC Bootcamp

> ✨ Day 5 - It's an Arbitrage Pirate's Life for Me

---

### [<img src="https://raw.githubusercontent.com/solana-developers/pirate-bootcamp/main/docs/images/slides-icon.svg" alt="slides" width="20" align="center"/> Presentation Slides (coming soon)](https://github.com/buffalojoec/nyc-bootcamp-arb-program)

### To Build:
* Arbitrage algorithm: `program/src/arb.rs#L12`
    * How to calculate & spot arbitrage opportunities between pools?
    * How to rank arbitrage opportunities against each other (which is higher value)?
    * How to check all combinations without max'ing out compute?
    * 💡 In order for our preflight client idea to work, the program must error if there's no suitable arbitrage opportunity detected
* Client-side code/tests
    * Preflights & auto-send transaction when opportunity is present
    * Log output demonstration similar to swap program
    * Lookup Table
    * Amman ?
* Arbitrage UI

### General Workshop Flow:
* Presentation Slides
    * How does arbitrage work?
    * Native program entrypoint & processors
* ✅ Inspect the program
    * ✅ Loading accounts
    * Detecting arbitrage opportunities
    * ✅ Placing trades
    * Preflight gamification
* Build & Deploy to localnet
* Run tests
    * Inspect logs
* ⚡️ UI
