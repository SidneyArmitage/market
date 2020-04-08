table! {
    company (id) {
        id -> Int4,
        name -> Varchar,
        dividend -> Date,
        shares -> Int4,
    }
}

table! {
    dividend (company, payment_date) {
        company -> Int4,
        payment_date -> Date,
        announcement_date -> Date,
        exdividend_date -> Date,
        #[sql_name = "dividend"]
        div -> Money,
    }
}

table! {
    industry (id) {
        beta -> Float8,
        name -> Varchar,
        id -> Int4,
    }
}

table! {
    industry_map (company) {
        company -> Int4,
        beta -> Float8,
        industry -> Int4,
    }
}

table! {
    industry_value (date) {
        date -> Date,
        value -> Float8,
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

joinable!(dividend -> company (company));
joinable!(industry_map -> company (company));
joinable!(ledger -> company (company));

allow_tables_to_appear_in_same_query!(
    company,
    dividend,
    industry,
    industry_map,
    industry_value,
    ledger,
);
