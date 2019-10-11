table! {
    members (member_id) {
        member_id -> Int4,
        name -> Varchar,
        knockouts -> Int4,
        team_id -> Int4,
    }
}

table! {
    teams (team_id) {
        team_id -> Int4,
        name -> Varchar,
    }
}

joinable!(members -> teams (team_id));

allow_tables_to_appear_in_same_query!(
    members,
    teams,
);
