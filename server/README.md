# Elastic Leaderboard Server

This subproject contains the server side component for the "Elastic Leaderboard" demo service using `smithy-rs`
for the server.


## Building

The `server-bootsrap` module contains a minimal Gradle build to generate the Rust server code. This
has to be built first before building the actual server.

```sh
cd server-bootstrap
./gradlew build
```

This will output the generated server stubs and shapes to `leaderboard-server-sdk`.

Now the service can actually be built.

```sh
cd elastic-leaderboard-service
cargo build
RUST_LOG=debug cargo run
```


