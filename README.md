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

TESTING

- Find column in pg_statistic that we want to modify
- Get the json dump with pg_statistic_dump
- Modify the attribute with pg_modify_stats
- Take the new json dump and load with pg_statistic_load
- Verify the statistic is updated
