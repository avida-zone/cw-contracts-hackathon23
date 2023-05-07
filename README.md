# Smart Contracts for AVIPAD

[AVIPAD](https://launchpad.avida.zone) is a token launchpad for regulatory grade tokens (rgTokens).
It implements the AVIDA (Atomic Verification of Identity proofs for Decentralised Applications) framework of using

- [AnonCreds] verifiable credentials
- An onchain verifier

to do ZKProof verification onchain for interactions with rgToken contracts.

The aim of this project is to bring a new form of regulated assets onchain,
increasing trading volumes and liquidity.

This repo holds the submission to the Injective 2023 Hackathon.

## Repository layout

### Contracts

- [avida-launchpad](./contracts/launchpad) -> A launchpad for new rgToken and transformer for any native non-rg tokens into Cw20 rg-tokens
- [rg-cw20](./contracts/rg-cw20) -> Regulatory Grade cw20 contract
- [vc-verifier](./contracts/vc-verifier) -> Onchain AnonCreds Verifiable Credential ZK compliance proofs verifier
- [identity-plugin](./contracts/identity-plugin) -> A plugin to the [Vectis Account] holding the rgAssets

RgTokens are implemented in a modified form of cw20, therefore on the [launchpad],
we also provide an adapter to turn rg-cw20 into native-rgTokens.
Please see the [adapter repo] for detail.

### Packages

- [avida-verifier](./packages/avida-verifier) -> holds shared types for usage of contracts above

### Tests

- [ts-tools](./ts-tools/tests) -> deploys all contracts to testnet, has ts types and tests interactions, simply run `npm run test`
- [avida-contracts-multitest](./packages/avida-contracts-multitest) -> holds contract multitest, run `cargo test`

[vectis account]: https://github.com/nymlab/vectis
[anoncreds]: https://hyperledger.github.io/anoncreds-spec/
[launchpad]: https://launchpad.avida.zone
[adapter repo]: https://github.com/avida-zone/cw20-adapter

## Implementaion details

### What is an identity plugin and why do we need it?

Consider an example, a verifier wants to know the holder 1) has a driver's license and 2) has a medical insurance.
The holder may proof they have both credentials but they were issued from 2 different issuers.
By default, Anoncreds allows ZK proof of ownership of the credential,
without revealing any other information such as name / age.
Without disclosing information about the holder,
how will the verifier know that the credentials have been issued to the same person?

On issuance of [Anoncreds] - a form of verifiable credential with ZK-Proof,
the holder (entity requesting the credential) must provide a masked "link_secret" to the issuer of the credential.
The "link_secret", in its plain form is a piece of cryptographic material that is only known to the holder.
The issuer blindly signs this "link_secret" as well as other unblinded attributes to form the
credential and shares the credntial to the holder.
The holder can then unblind the credential and stores it.

When the holder wants to create proof of ownerships,
the holder NEVER shares their credential,
the holder can now proof that the "link_secret" provided to the different issuers is the same.
Therefore linking those credentials.

#### How is link_secret used in AVIDA

Because the verification of the credential proofs is done onchain in a public manner,
[Anoncreds] is the best form of verifiable credential to use.

However we face the same problem,
how do we know that the drivers license credential connects to the caller of the onchain transactions, Alice?

The same solution applies,
if we can proof that Alice knows the "link_secret" to

1. the driver license credential and
2. a credential stating Alice owns the private key to the onchain account,

then we can proof that the drivers license is linked an account onchain.

In this case, Alice must self issue a credential using the same "link_secret" stating that she owns the pubkey / addr onchain.
This means that Alice becomes both the holder of a credential and an issuer of the self-issued credential.

This plugin allows Alice to share the pubkey to any verifier, as an issuer.

[AnonCreds]: https://hyperledger.github.io/anoncreds-spec

---

Copyright 2023 NYMLAB Srl
