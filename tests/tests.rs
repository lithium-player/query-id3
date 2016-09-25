extern crate liquery;
extern crate liquery_id3;

use liquery::Queryable;
use liquery_id3::QueryId3;

macro_rules! query_test {
    ($name: ident, $field: expr, $out: expr) => {
        #[test]
        fn $name() {
            let queryable = QueryId3::new("tests/test.mp3").unwrap();

            assert_eq!(Some($out.to_owned()), queryable.query($field));
        }
    };
}

query_test!(title       , "title"       , "test");
query_test!(artist      , "artist"      , "liquery");
query_test!(album       , "album"       , "file");
query_test!(album_artist, "album artist", "id3");
query_test!(year        , "year"        , "2016");
query_test!(disc_number , "disc number" , "1");
query_test!(total_discs , "total discs" , "2");
query_test!(total_tracks, "total tracks", "3");
query_test!(track_number, "track number", "2");
