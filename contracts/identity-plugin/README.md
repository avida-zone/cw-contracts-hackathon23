# Identity Plugin

This plugin is designed to be pluggable as a Queriable Plugin to the [Vectis] smart contract wallet.

## Why do we need this plugin?

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

## How is link_secret used in AVIDA

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

[vectis]: https://github.com/nymlab/vectis
[anoncreds]: https://github.com/nymlab/vectis
