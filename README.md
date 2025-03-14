Running project

1. Install pgrx
2. Run `cargo pgrx init`
   If you see "ICU lib not found: install pkg-config and
   run `export PKG_CONFIG_PATH=/opt/homebrew/opt/icu4c/lib/pkgconfig` before command

3. Run `cargo build`
   If you get error "linking with `cc` failed, add these lines to Cargo.toml:

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
