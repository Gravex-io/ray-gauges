# Raydium Redux

This collection of programs adds a significant range of new features to Raydium, mostly centering around utility for the RAY token.

## Program overview

### Reactor

The `reactor` program is an escrow account for RAY tokens. This program converts RAY tokens into **vote power** (with the addition of `isoRAY` as a time-based component). The `reactor` program also provides "staking rewards" to RAY holders.

### CP LP Escrow

The `cp_lp_escrow` program is a singular escrow program for holding balances of constant-product swap (CP-Swap) LP tokens. The reason for a central program to hold balances of LP tokens is that this balance will be used for calculating `gauge` rewards (RAY emissions) and eventually "ecofarm" rewards (arbitrary emissions).

### Gauge

The `gauge` program handles RAY emissions for CP and CL markets. Gauges are voted on using `reactor` votes.

The `gauge` program has several dependecies on other programs:

- when voting on a gauge, or releasing votes, the `gauge` program locks/unlocks votes in the `reactor` program
- when calculating the amount of RAY to emit, the `gauge` program reads the quantity of liquidity tokens. For CP Swap, this dependency is with the `cp_lp_escrow` program. For CL pools, the `gauge` program reads from the pool state of the CLMM.

## Dev

Check dependencies:

```
$ anchor --version
anchor-cli 0.29.0

$ solana --version
solana-cli 1.18.2 (src:13656e30; feat:3352961542, client:SolanaLabs)
```

Install NPM packages:

```
$ yarn
```

Run tests (must compile related TypeScript packages first)

```
$ cd sol
$ tsc --build
$ anchor test -- --features localnet
$ cargo test
```

### Client generation

We auto-generate TypeScript clients from the Anchor artifacts.

```
$ cd sol
$ anchor build
$ yarn gen
```
