CREATE TABLE issuers (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  correctness TEXT NOT NULL,
  privkey TEXT NOT NULL,
  subproofreqparams TEXT NOT NULL
)
