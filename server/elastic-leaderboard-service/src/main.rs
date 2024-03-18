use crate::leaderboards::{
    create_leaderboard_handler, delete_leaderboard_handler, get_leaderboard_handler,
    list_leaderboards_handler, submit_score_event_handler, update_leaderboard_handler, State,
};
use aws_smithy_http_server::{
    extension::OperationExtensionExt, plugin::HttpPlugins,
    instrumentation::InstrumentExt,
    request::request_id::ServerRequestIdProviderLayer, AddExtensionLayer,
};
use clap::Parser;
use leaderboard_server_sdk::{ElasticLeaderboardService, ElasticLeaderboardServiceConfig};
use std::{net::SocketAddr, sync::{Arc, Mutex}};
use tracing_subscriber::{prelude::*, EnvFilter};

mod cli;
mod leaderboards;

/// Setup `tracing::subscriber` to read the log level from RUST_LOG environment variable.
pub fn setup_tracing() {
    let format = tracing_subscriber::fmt::layer().json();
    let filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();
    tracing_subscriber::registry()
        .with(format)
        .with(filter)
        .init();
}

#[tokio::main]
pub async fn main() {
    let args = cli::Args::parse();
    setup_tracing();

    let http_plugins = HttpPlugins::new()
        .insert_operation_extension()
        // add tracing spans and events to request lifecycle
        .instrument();

    let state = Arc::new(Mutex::new(State::default()));
    let config = ElasticLeaderboardServiceConfig::builder()
        .layer(AddExtensionLayer::new(state))
        // add server request IDs
        .layer(ServerRequestIdProviderLayer::new())
        .http_plugin(http_plugins)
        .build();

    let svc = ElasticLeaderboardService::builder(config)
        .submit_score_event(submit_score_event_handler)
        .create_leaderboard(create_leaderboard_handler)
        .get_leaderboard(get_leaderboard_handler)
        .update_leaderboard(update_leaderboard_handler)
        .delete_leaderboard(delete_leaderboard_handler)
        .list_leaderboards(list_leaderboards_handler)
        .build()
        .expect("failed to build an instance of ElasticLeaderboardService");

    println!("starting elastic leaderboard service...");
    println!("listening on {:?}", args);

    let make_app = svc.into_make_service_with_connect_info::<SocketAddr>();

    // Bind the application to a socket.
    let bind: SocketAddr = format!("{}:{}", args.address, args.port)
        .parse()
        .expect("unable to parse the server bind address and port");
    let server = hyper::Server::bind(&bind).serve(make_app);

    // Run forever-ish...
    if let Err(err) = server.await {
        eprintln!("server error: {}", err);
    }
}
