CREATE TABLE credentials (
  id SERIAL PRIMARY KEY,
  contractaddr VARCHAR NOT NULL,
  issuer VARCHAR NOT NULL,
  walletaddr VARCHAR NOT NULL,
  credsig TEXT NOT NULL,
  credvalues TEXT NOT NULL
)
