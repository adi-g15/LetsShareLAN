use dotenv;
use mysql::{self, prelude::Queryable, Conn, OptsBuilder};
use whoami;

/*
 * To be able to use MariaDB/MySQL. You MUST have a .env file containing:
 *
 * SQL_USERNAME=your-sql-username
 * SQL_PASSWORD=your-sql-password
 * */

static SQL_FAILED_ERROR_ENV: &str =
    "Failed to get environment variable. If you don't need SQL, use --nosql flag.";
static SQL_FAILED_ERROR_CONN: &str =
    "Failed connecting to SQL. If you don't need SQL, use --nosql flag.";
static SQL_FAILED_ERROR_QUERY: &str =
    "Failed quering SQL. If you don't need SQL, use --nosql flag.";

fn sql_get_ids() -> Vec<(String, String)> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    let sql_username = match dotenv::var("SQL_USERNAME") {
        Ok(uname) => uname,
        Err(err) => {
            println!("ERROR: {}", SQL_FAILED_ERROR_ENV);
            println!("DETAILED ERROR: {:?}", err);

            // trying the user's own username
            println!(
                "WARN: Using your current username: {} as sql username",
                whoami::username()
            );
            whoami::username()
        }
    };

    // Password can be None
    let sql_password = dotenv::var("SQL_PASSWORD").ok();

    let db_name = match dotenv::var("DB_NAME") {
        Ok(db) => Some(db),
        Err(_) => {
            println!("WARN: Using default database, as DB_NAME not provided");
            None
        }
    };

    let opts = OptsBuilder::new()
        .user(Some(sql_username))
        .pass(sql_password)
        .db_name(db_name);

    let mut conn = Conn::new(opts).expect(SQL_FAILED_ERROR_CONN);

    conn.query_map(
        "SELECT username, pwd FROM passwords",
        |(username, pwd)| {
            (username, pwd)
        })
        .expect(SQL_FAILED_ERROR_QUERY)
}

fn main() {
    let use_sql = true;

    let ids = if use_sql {
        sql_get_ids()
    } else {
        vec![]
        // Use passed username password
    };

    println!("{:?}", ids);
}
