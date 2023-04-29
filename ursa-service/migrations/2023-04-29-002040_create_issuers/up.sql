CREATE TABLE issuers (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  correctness TEXT NOT NULL,
  privkey TEXT NOT NULL,
  pubkey TEXT NOT NULL,
  schema TEXT NOT NULL,
  nonschema TEXT NOT NULL,
  subproofreq TEXT NOT NULL
)
