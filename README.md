# Atomically Verifiable Identity Decentralised Application

AVIDA is a framework for decentralised trusted applications that are regulatory grade.

The aim of this project is to bring a new form of regulated assets onchain,
increasing trading volumes and liquidity.

This repo holds the submission to the Injective 2023 Hackathon.

## Repository layout

- contracts

  - [avida-launchpad] -> A launchpad for new rgToken and transformer for any native non-rg tokens into Cw20 rg-tokens
  - [rg-cw20] -> Regulatory Grade cw20 contract
  - [vc-verifier] -> Onchain AnonCreds Verifiable Credential ZK compliance proofs verifier
  - [identity-plugin] -> A plugin to the Vectis Account holding the rgAssets

- packages

  - [avida-verifier] -> holds shared types for usage of contracts above

- tests
  - [avida-contracts-multitest] -> holds contract multitest
  - [ts-tools] -> deploys all contracts to testnet, has ts types and tests interactions
