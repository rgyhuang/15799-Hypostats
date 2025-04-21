Running project

1. Install pgrx
2. Run `cargo pgrx init`
   If you see "ICU lib not found: install pkg-config and
   run `export PKG_CONFIG_PATH=/opt/homebrew/opt/icu4c/lib/pkgconfig` before command

3. Run `cargo build`
   If you get error "linking with `cc` failed, add these lines to `~/.cargo/config`:

   ```
       [target.x86_64-apple-darwin]
       rustflags = [
       "-C", "link-arg=-undefined",
       "-C", "link-arg=dynamic_lookup",
       ]

       [target.aarch64-apple-darwin]
       rustflags = [
       "-C", "link-arg=-undefined",
       "-C", "link-arg=dynamic_lookup",
       ]
   ```

4. When making changes, after compiling must drop the extension and re-add it with
   `DROP EXTENSION hypostats` and `CREATE EXTENSION hypostats`
5. Run `Select install_size_hook()` and `select remove_size_hook()` to install/drop hook

Running backend server

1. Modify the postgres connection in the main function of main.rs to connect
   - To find the correct port, you can run `cargo pgrx run` and it will say: Starting
     Postgres vXXX on port XXX
   - To find the username, run `\du` in the Postgres terminal
2. Run `cargo run --bin hypostats` to get the backend up
3. Make http requests from localhost on port 8080 like so to get pg_statistic dumps:
   - curl localhost:8080/query -d '{ "starelid": 41281, "staattnum": 1 }'
4. When testing the load API, run curl with the -g flag to allow for the nested brackets
5. If any updates are made to lib.rs, make sure to drop the extension and create it
   again before building again
