#[macro_use]
extern crate diesel;

use diesel::prelude::*;

mod types {
    use diesel::deserialize::Queryable;
    use diesel::sqlite::SqliteConnection;

    table! {
        records (iid) {
            iid -> Integer,
            id_0 -> BigInt,
            id_1 -> BigInt,
            data -> Text,
        }
    }

    #[derive(Debug, Copy, Clone, FromSqlRow)]
    pub struct RecordId {
        id_0: i64,
        id_1: i64,
    }

    // Using a RecordId in a Record compiles, but 
    // produces an error when used in an actual query
    #[derive(Queryable, Debug)]
    pub struct Record {
        pub iid: i32,
        pub id: RecordId,
        pub data: String,
    }


    // Manually implementing Queryable works, 
    // but doesn't scale well, since every change to the schema
    // requires a rewrite of the implementation, and I could not
    // work out how to make the implementation work for a generic DB
    // type
    #[derive(Debug)]
    pub struct RecordManual {
        pub iid: i32,
        pub id: RecordId,
        pub data: String,
    }

    impl Queryable<records::SqlType, diesel::sqlite::Sqlite> for RecordManual 
    {
        type Row = (i32, i64, i64, String);
        fn build(row: Self::Row) -> Self {
            RecordManual {
                iid: row.0,
                id: RecordId {
                    id_0: row.1,
                    id_1: row.2,
                },
                data: row.3,
            }
        }
    }


    // Just using the expanded fields works, but is not very ergonomic.
    #[derive(Queryable, Debug)]
    pub struct RecordDirect {
        pub iid: i32,
        pub id_0: i64,
        pub id_1: i64,
        pub data: String,
    }
}

use types::records::dsl::*;

pub fn find(connection:&SqliteConnection) -> types::Record {
    records.find(1).get_result::<types::Record>(connection).unwrap()
}

pub fn find_manual(connection:&SqliteConnection) -> types::RecordManual {
    records.find(1).get_result::<types::RecordManual>(connection).unwrap()
}

pub fn find_direct(connection:&SqliteConnection) -> types::RecordDirect {
    records.find(1).get_result::<types::RecordDirect>(connection).unwrap()
}

fn main() {
}
