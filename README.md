# Atomically Verifiable Identity Decentralised Application

AVIDA is a framework for decentralised trusted applications that are regulatory grade.

The aim of this project is to bring a new form of regulated assets onchain,
increasing trading volumes and liquidity.

This repo holds the submission to the Injective 2023 Hackathon.

## Repository layout

- contracts

  - [rg-cw20] -> Regulatory Grade cw20 contract
  - [vc-verifier] -> Onchain AnonCreds Verifiable Credential ZK compliance proofs verifier
  - [identity-plugin] -> A plugin to the Vectis Account holding the rgAssets
  - [asset-transformer] -> A transformer for any native non-rg tokens into Cw20 rg-tokens
  - [rg-asset-adaptor] -> An adaptor inspired by the Injective Adaptor, allows for Cw20 rg-token <=> native tokens on Injective to facilitate usage with applications on Injective

- packages

  - [avida-verifier] -> holds shared types for usage of contracts above

- [ursa-demo]: A demo binary to mock the setup of issuers of Anoncreds and self-issuers by the rgAssets holder
- [ursa-service]: A server to mock the user flow
