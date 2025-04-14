# Universal Liquidity Pool Parser
`sail-level` framework (just kidding)

Provides a universal interface for parsing liquidity pools on Solana. It offers a struct through which you can access various fields, e.g., quote_vaults.

## Currently supports:

### Raydium:

- Standard AMM (CP-Swap, New): `CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C`
- Concentrated Liquidity (CLMM): `CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK`

### (New) Pump.fun Liquidity Pools
- Pump Swap AMM: `pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA`

## Features to Add
- Support for additional protocols
- Improved error handling and logging
- Enhanced documentation and examples
- Parsing from JSON
- Parsing from Bytes
