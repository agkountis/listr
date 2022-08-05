use diesel::{Connection, PgConnection};

const DATABASE_URL: &str = "postgres://postgres:1234@localhost/listr";

pub fn establish_connection() -> PgConnection {
    PgConnection::establish(DATABASE_URL).expect(&format!("Error connecting to {}", DATABASE_URL))
}
