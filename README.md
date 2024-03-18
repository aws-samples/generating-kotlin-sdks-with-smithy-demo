# Generating Kotlin SDKs with Smithy

This repository contains the full working example code that was used in the 
[KotlinConf 2024](https://kotlinconf.com/2024/) talk "Generating Kotlin SDKs with Smithy"

## Quickstart

### Bootstrap client and server

First the client and server projections need generated:

```sh
# bootstrap the client
./gradlew -p client/client-bootstrap build

# bootstrap the server
./gradlew -p server/server-bootstrap build
```

This will create the `client/leaderboard-client-sdk` and `server/leaderboard-server-sdk` directories with the
generated code.

### Start the server

```sh
cd server/elastic-leaderboard-service
cargo run
```

### Run the client

```sh
./gradlew -p client/leaderboard-client run
```

## Security

See [CONTRIBUTING](CONTRIBUTING.md#security-issue-notifications) for more information.

## License

This library is licensed under the MIT-0 License. See the LICENSE file.
