use std::{
    error, env::temp_dir,
    fs, path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH}, io::Write,
};

use debug::debugln;
use dialoguer;
use dotenv;
use mysql::{self, prelude::Queryable, Conn, OptsBuilder};
use rand::prelude::*;
use reqwest::blocking::Client;
use urlencoding::encode;
use whoami;

static BASE_URL: &str = "http://172.172.172.100:8090/";


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

    let db_name = dotenv::var("DB_NAME")
        .expect("DB_NAME environment variable is mandatory, and must contain the database name");

    let opts = OptsBuilder::new()
        .user(Some(sql_username))
        .pass(sql_password)
        .db_name(Some(db_name));

    let mut conn = Conn::new(opts).expect(SQL_FAILED_ERROR_CONN);

    conn.query_map("SELECT username, pwd FROM passwords", |(username, pwd)| {
        (username, pwd)
    })
    .expect(SQL_FAILED_ERROR_QUERY)
}

/**
 * @return Returns number of milliseconds since UNIX/ECMAScript epoch, ie. 1 Jan 1970
 */
fn get_milliseconds_since_epoch() -> u128 {
    let now = SystemTime::now();

    now.duration_since(UNIX_EPOCH)
        .expect("Time went backwards :O")
        .as_millis()
}

/**
 * Log in to the captive portal using the provided username and password
 */
fn login_user(username: String, password: String) -> Result<(), Box<dyn error::Error>> {
    const FAILED_MESSAGE: &str = "Make sure your password is correct";
    let tmp_filepath: PathBuf = temp_dir().join("lsl.username");

    if Path::exists(&tmp_filepath) {
        let old_username = fs::read_to_string(&tmp_filepath).unwrap();
        println!("WARN: Possibly already logged in with username={}", old_username);
    }

    let login_url = BASE_URL.to_string() + "login.xml";

    let millis = get_milliseconds_since_epoch();

    /*
     * Mode: 191 (for login), 193 (for logout)
     * ProductType: 0 (desktop), 1 (iPhone/iPad), 2 (Android)
     */
    let query = format!(
        "mode=191&username={}&password={}&a={}&producttype=0",
        encode(username.as_str()),
        encode(password.as_str()),
        millis
    );

    let client = Client::new();

    /* The headers are not required, but I am adding them just for possible stability later */
    let res = client
        .post(login_url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Referer", BASE_URL)
        .body(query)
        .send()?;

    debugln!("{:?}", res);

    let response = res.text()?;
    debugln!("{:?}", response);

    if response.contains(FAILED_MESSAGE) {
        Err("Failed to login")?
    } else {
        // cache username to file
        if Path::exists(&tmp_filepath) {
            fs::remove_file(&tmp_filepath)?;
        }
        let mut file = fs::File::create(&tmp_filepath)?;
        file.write(username.as_bytes())?;
        Ok(())
    }
}

fn logout_user(username: String) -> Result<(), reqwest::Error> {
    println!("Logging out user: {}", username);
    let logout_url = BASE_URL.to_string() + "logout.xml";

    let millis = get_milliseconds_since_epoch();

    /*
     * Mode: 191 (for login), 193 (for logout)
     * ProductType: 0 (desktop), 1 (iPhone/iPad), 2 (Android)
     */
    let query = format!(
        "mode=193&username={}&a={}&producttype=0",
        encode(username.as_str()),
        millis
    );

    let client = Client::new();

    /* The headers are not required, but I am adding them just for possible stability later */
    let res = client
        .post(logout_url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Referer", BASE_URL)
        .body(query)
        .send()?;

    debugln!("{:?}", res);
    debugln!("{:?}", res.text());

    Ok(())
}

fn main() -> Result<(), Box<dyn error::Error>> {
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

    let tmp_filepath: PathBuf = temp_dir().join("lsl.username");
    if should_logout {
        let username = if Path::exists(&tmp_filepath) {
            // Read username from tmp_filepath (eg. /tmp/lsl.username)
            fs::read_to_string(tmp_filepath.to_str().unwrap()).unwrap()
        } else {
            // random username also works for logout in our captive portal
            "111111".to_string()
        };

        logout_user(username)?;

        // Remove when logged out
        if Path::exists(&tmp_filepath) && fs::remove_file(&tmp_filepath).is_err() {
            println!("WARN: Failed to remove {:?}", tmp_filepath.to_str());
        }

        return Ok(());
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
        print!("Trying to login with {}... ", cred.0);

        if login_user(cred.0, cred.1).is_ok() {
            println!("Succeeded");
            connected = true;
            break;
        } else {
            println!("Failed");
        }
    }

    if !connected {
        Err("Failed to login. Please check the connection/credentials.")?;
    }

    Ok(())
}
