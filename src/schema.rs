table! {
    company (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    ledger (id) {
        id -> Int4,
        company -> Int4,
        is_buy -> Bool,
        price -> Money,
        quantity -> Int4,
        buyer -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    company,
    ledger,
);
