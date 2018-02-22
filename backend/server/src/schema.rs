// $ diesel print-schema, then copy-paste it here.
table! {
    ads (id) {
        id -> Text,
        html -> Text,
        political -> Int4,
        not_political -> Int4,
        title -> Text,
        message -> Text,
        thumbnail -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        lang -> Text,
        images -> Array<Text>,
        impressions -> Int4,
        political_probability -> Float8,
        targeting -> Nullable<Text>,
        suppressed -> Bool,
        targets -> Nullable<Jsonb>,
        advertiser -> Nullable<Text>,
        entities -> Nullable<Jsonb>,
        page -> Nullable<Text>,
    }
}

table! {
    candidates (facebook_url) {
        facebook_url -> Text,
        name -> Text,
        state -> Nullable<Varchar>,
        district -> Nullable<Varchar>,
        race_type -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    ads,
    candidates,
);
