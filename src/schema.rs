table! {
    company (id) {
        id -> Int4,
        name -> Varchar,
        dividend -> Date,
        shares -> Int4,
        stdev -> Float8,
    }
}

table! {
    dividend (company, payment_date) {
        company -> Int4,
        payment_date -> Date,
        announcement_date -> Date,
        exdividend_date -> Date,
        payment -> Float8,
    }
}

table! {
    industry (id) {
        id -> Int4,
        name -> Varchar,
        beta -> Float8,
        stdev -> Float8,
    }
}

table! {
    industry_map (industry, company) {
        industry -> Int4,
        company -> Int4,
        beta -> Float8,
        weight -> Float8,
    }
}

table! {
    industry_value (industry, date) {
        industry -> Int4,
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

table! {
    projected_dividend (company, value_date) {
        company -> Int4,
        index -> Float8,
        value_date -> Date,
    }
}

joinable!(dividend -> company (company));
joinable!(industry_map -> company (company));
joinable!(industry_map -> industry (industry));
joinable!(industry_value -> industry (industry));
joinable!(ledger -> company (company));
joinable!(projected_dividend -> company (company));

allow_tables_to_appear_in_same_query!(
    company,
    dividend,
    industry,
    industry_map,
    industry_value,
    ledger,
    projected_dividend,
);
