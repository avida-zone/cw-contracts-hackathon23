// @generated automatically by Diesel CLI.

diesel::table! {
    credentials (id) {
        id -> Int4,
        contractaddr -> Varchar,
        issuer -> Varchar,
        walletaddr -> Varchar,
        credsig -> Text,
        credvalues -> Text,
    }
}

diesel::table! {
    issuers (id) {
        id -> Int4,
        name -> Varchar,
        correctness -> Text,
        privkey -> Text,
        subproofreqparams -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    credentials,
    issuers,
);
