# Let's Share LAN

![LetsShareLAN](https://socialify.git.ci/adi-g15/LetsShareLAN/image?description=1&language=1&logo=https%3A%2F%2Fupload.wikimedia.org%2Fwikipedia%2Fcommons%2Fthumb%2F2%2F20%2FRustacean-orig-noshadow.svg%2F200px-Rustacean-orig-noshadow.svg.png&name=1&owner=1&pattern=Circuit%20Board&theme=Dark)

<div align="center">
  <a href="https://github.com/adi-g15/LetsShareLAN/actions/workflows/rust.yml"><img alt="Build Status" src="https://github.com/adi-g15/LetsShareLAN/actions/workflows/rust.yml/badge.svg" /></a>
</div>

Command to automate logging in to captive portal.

Currently supports 3 modes:
1. SQL: Get credentials from a sql table (can be using UNIX socket, or using
   URL)
2. File: Read credentials from a passwords file at `$HOME/lsl.pwd`
3. Manual: Ask user for login credentials

> TIP:
>
> I use this mostly by **setting it to start automatically after boot**.
>
> Steps with systemd (linux):
>
> 1. First install this using `cargo install --root /usr --path . --no-track`. Or use the PKGBUILD file, if on Arch Linux.
> 2. Copy lets-share-lan.service to ~/.local/share/systemd/user/lets-share-lan.service. EDIT the service file with correct environment variables, and in After="<xxxxx>", put either `mysql.service` or `mariadb.service`
> 3. systemctl --user enable --now lets-share-lan
>
> Done... FORGET ABOUT LOGGING IN AGAIN MANUALLY :)

### PreRequisite

Install rust from [here](https://rustup.rs).

Then just clone this repo with git, and see [usage section](#usage)

```sh
cargo build
```

### Usage

1. Logging in:

By default it will fetch passwords from a MySQL database.

So first see the .env_sample file, and create corresponding .env file.

Ensure that your SQL server is running (mariadb, mysql, default port 3306). Create a database, and enter the name of database in the `DB_NAME` environment variable (or in .env), eg. `DB_NAME=wifi`.

Then create a `passwords` table like this:

```sql
create database wifi;
use wifi;

create table passwords (username varchar(10), pwd varchar(50));

% Now insert known credentials in this table, LetsShareLAN will then randomly chose among these each time
insert into …
```

```sh
cargo run
```

1.2. Login by manually entering username,password

```sh
cargo run -- --manual
```

1.3. Login by using a passwords file in home directory (ie. `$HOME/lsl.pwd`)

```sh
cargo run -- --usefile
```

The file at `$HOME/lsl.pwd` should have content like this:

```toml
username = password
complex-username = complex-password@123
```

2. Logout:

```sh
cargo run -- logout
```

### Idea

There is a data limit I always exhausted, while many of my friends didn't, so I can use their IDs, this is my way to automate that :)

> Note to self: Bhai kuchh aur bhi ideas the, jaise ye sb VPN/TOR ke through secure ho... pta nhi 1st yr me itna kyu aur kaise sochha, shabaas ! 😂
>
>               To see old code, see initial 2 commits. Back then, used selenium, and MANY good inferences

