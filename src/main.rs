use std::{fs, path::Path};

use rand::prelude::*;
use dialoguer;
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

fn sql_get_credentials() -> Vec<(String, String)> {
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

fn login_user(username: String, password: String) -> Result<(),()> {
    println!("Logging in user: {}", username);

    Ok(())
}

fn logout_user(username: String) -> Result<(),()> {
    println!("Logging out user: {}", username);

    Ok(())
}

fn main() -> Result<(),()> {
    let args = std::env::args().collect::<Vec<_>>();

    if args.contains(&"--help".to_string()) {
        println!("Usage: lsl [login/logout] [--nosql]");
        println!("\tlogin: [Default] Login with given/sql fetched credentials");
        println!("\t\t--nosql: Don't use SQL (for getting login credentials)");
        println!("\tlogout: Logout from the current session");

        return Ok(());
    }

    let should_logout = args.contains(&"logout".to_string());
    let use_sql = !args.contains(&"--nosql".to_string());

    if should_logout {
        println!("Logging out...");

        let username = if Path::try_exists(&Path::new("/tmp/lsl.username")).is_ok() {
            // Read username from /tmp/lsl.username
            fs::read_to_string("/tmp/lsl.username").unwrap()
        } else {
            dialoguer::Input::new()
                .with_prompt("Enter logged in username")
                .interact()
                .unwrap()
        };

        logout_user(username)?;
    }

    let mut credentials = if use_sql {
        sql_get_credentials()
    } else {
        println!("Read instructions at: https://github.com/adi-g15/LetsShareLAN");

        let username = dialoguer::Input::new()
            .with_prompt("Enter your NITP username: ")
            .interact_text()
            .unwrap();
        let password = dialoguer::Password::new()
            .with_prompt("Enter your NITP password: ")
            .interact()
            .unwrap();

        vec![(username, password)]
    };

    // random shuffle credentials
    let mut rng = thread_rng();
    credentials.shuffle(&mut rng);

    let mut connected = false;
    for cred in credentials {
        println!("Trying to login with {}... ", cred.0);

        if login_user(cred.0, cred.1).is_ok() {
            println!("Failed")
        } else {
            println!("Succeeded");
            connected = true;
            break;
        }
    }

    if ! connected {
        println!("Failed to login. Please check the connection/credentials.");
        return Err(());
    }

    Ok(())
}
