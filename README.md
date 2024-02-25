# SoundGoodDB
This is a database contained in /sql and a CLI built in Rust using sqlx to communicate with the database.
The CLI does not use all features of the database but does use all features related to the rental part of the system.

## To setup
1. Pull down the repo
1. Access your postgres database and run the .sql files under /sql. Either run create then insert or run test.sql which will reset the database if it exists and create it in a transaction which has to be commited.
1. In the root folder (next to /src and /sql) create a file called `.env` and in it add one line `DATABASE_URL = "postgres://user:pass@localhost:5432/sgdb"` where `user` is your postgres username, `pass` your postgres password and `sgdb` is the name of the database.
1. ```cargo run --release``` and let rustc build and execute the program.

## Notes
- The program uses sqlx's macros for compile time checking etc and will not compile unless you configure it differently or have the `.env DATABASE_URL` accessible database running when compiling.
- You can run ```cargo test``` to run the tests included in the program, which also need to access the database.
- You can run ```cargo rustdoc``` to generate the documentation for the program.

