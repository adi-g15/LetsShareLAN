use std::{
    error, env::temp_dir,
    fs, path::{Path, PathBuf},
    time::{Duration, SystemTime, UNIX_EPOCH, Instant}, io::{Read, Write},
    mem, thread::sleep
};

use debug::debugln;
use mysql::{self, prelude::Queryable, Conn, OptsBuilder};
use rand::prelude::*;
use reqwest::blocking::Client;
use urlencoding::encode;
use online::check;
use single_instance::SingleInstance;
use home::home_dir;

// Put your college/company LAN login page URL
static BASE_URL: &str = "http://172.172.172.100:8090/";

/*
 * To be able to use MariaDB/MySQL. You MUST have a .env file containing:
 *
 * SQL_USERNAME=your-sql-username
 * SQL_PASSWORD=your-sql-password
 * */

static SQL_FAILED_ERROR_ENV: &str =
    "Failed to get environment variable. If you don't need SQL, use --manual or --usefile flag.";
static SQL_FAILED_ERROR_CONN: &str =
    "Failed connecting to SQL. If you don't need SQL, use --manual or --usefile flag.";
static SQL_FAILED_ERROR_QUERY: &str =
    "Failed quering SQL. If you don't need SQL, use --manual or --usefile flag.";

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

    const MARIADB_SOCKET_PATH: &str = "/run/mysqld/mysqld.sock";
    let opts = if Path::exists(Path::new(MARIADB_SOCKET_PATH)) {
        opts.prefer_socket(true)   // preferring sockets, since I use mariadb
                                   // only on localhost and disabled networking
            .socket(Some(MARIADB_SOCKET_PATH.to_string()))
    } else {
        opts
    };

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
fn login_user(username: &str, password: &str) -> Result<(), Box<dyn error::Error>> {
    const FAILED_MESSAGE: &str = "Make sure your password is correct";
    const DATA_EXCEEDED_MSG: &str = "data transfer has been exceeded";

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
        encode(username),
        encode(password),
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
    } else if response.contains(DATA_EXCEEDED_MSG) {
        Err("Data limit exceeded")?
    } else {
        // cache username to file
        if Path::exists(&tmp_filepath) {
            fs::remove_file(&tmp_filepath)?;
        }
        let mut file = fs::File::create(&tmp_filepath)?;
        file.write_all(username.as_bytes())?;
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

// Main logic, ensures only one copy is running preferring the latest one
fn daemon(credentials: &mut Vec<(String,String)>) -> Result<(), Box<dyn error::Error>> {
    // Required for SingleInstance to create some socket etc
    #[allow(unused_assignments)]
    let mut single_instance = SingleInstance::new("lsl").unwrap();

    const MAX_TRIES: i32 = 20;

    let mut tries = MAX_TRIES;
    let mut last_try = Instant::now();
    loop {
        // random shuffle credentials
        let mut rng = thread_rng();
        credentials.shuffle(&mut rng);

        if tries == 0 {
            println!("WARN: Tried {} times, sleeping for 20 minutes", MAX_TRIES);
            sleep(Duration::from_secs(60*20));
            tries = MAX_TRIES;
        }

        // If the last try was less than 10 seconds ago, count as a try
        if Instant::now().duration_since(last_try).as_secs() < 10 {
            tries = tries-1;
        } else {
            tries = MAX_TRIES;
        }

        // set last try
        last_try = Instant::now();

        let mut succeeded = false;
        for cred in credentials.iter() {
            print!("Trying to login with {}... ", cred.0);

            if login_user(&cred.0, &cred.1).is_ok() {
                println!("Succeeded");
                succeeded = true;
                break;
            } else {
                println!("Failed");
            }
        }

        if !succeeded {
            println!("WARN: Failed to login. Please check the connection/credentials.");
        } else {
            // Sleep for 30s connection has succeeded, wait for some time before moving on
            // code
            sleep(Duration::from_secs(30));
        }

        // call SingleInstance's destructor, so it can clean up sockets etc, before allocating next
        // resource
        mem::drop(single_instance);
        // refresh SingleInstance instance, or else it will keep using the state of initialisation
        single_instance = SingleInstance::new("lsl").unwrap();

        if !single_instance.is_single() {
            println!("Another instance is already running, exiting...");
            break Ok(());
        }
 
        // see, if we are connected, if so sleep for 5 minutes
        while check(None).is_ok() {
            println!("Already connected...");
            sleep(Duration::from_secs(60*5));
        }

        println!("Will try again to connect...");
   }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let args = std::env::args().collect::<Vec<_>>();

    if args.contains(&"--help".to_string()) {
        println!("Usage: lsl [login/logout] [--ask/--usefile]");
        println!("\tlogin: [Default] Login with given/sql fetched credentials");
        println!("\t\t--manual: Ask user for login credentials (No SQL)");
        println!("\t\t--usefile: Use credentials from $HOME/lsl.toml (No SQL, Plaintext)");
        println!("\tlogout: Logout from the current session");

        return Ok(());
    }

    let should_logout = args.contains(&"logout".to_string());
    let manual_cred = args.contains(&"--manual".to_string());
    let use_file = args.contains(&"--usefile".to_string());

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

    let mut credentials = if use_file {
        // Read credentials from $HOME/lsl.toml
        let cred_filepath = home_dir().expect("Could not get the HOME directory path.").join("lsl.toml");

        // If `cred_filepath` doesn't exist, opening it will fail since it tries in read-only mode
        let mut file = fs::File::open(&cred_filepath).expect(&format!("Could not open {}. Check kar le hai bhi ki nahi :)", cred_filepath.display()));

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let mut credentials = Vec::new();
        for line in contents.lines() {
            if line.is_empty() { continue; }

            let split = line.split_once('=').expect("Invalid credentials file format. Each line should be like \"username\" = \"password\"");
            let username = split.0.trim().to_string();
            let password = split.1.trim().to_string();

            credentials.push((username, password));
        }

        credentials
    } else {
        // @adig Set manual input as default behaviour in this variant
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

    daemon(&mut credentials)
}
