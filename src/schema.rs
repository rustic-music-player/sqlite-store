table! {
    albums (id) {
        id -> Integer,
        title -> Text,
        artist_id -> Nullable<Integer>,
        coverart -> Nullable<Text>,
        column_5 -> Nullable<Integer>,
    }
}

table! {
    artists (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    tracks (id) {
        id -> Integer,
        title -> Text,
        artist_id -> Nullable<Integer>,
        album_id -> Nullable<Integer>,
        stream_url -> Text,
        uri -> Text,
        coverart -> Nullable<Text>,
        duration -> Nullable<Integer>,
        column_9 -> Nullable<Integer>,
    }
}

joinable!(albums -> artists (artist_id));
joinable!(tracks -> albums (album_id));
joinable!(tracks -> artists (artist_id));

allow_tables_to_appear_in_same_query!(
    albums,
    artists,
    tracks,
);
