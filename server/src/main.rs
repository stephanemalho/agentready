use anyhow::Context;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .context("PORT must be a valid port number")?;

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port))
        .await
        .with_context(|| format!("failed to bind port {port}"))?;

    println!("agentready-server listening on 0.0.0.0:{port}");

    axum::serve(listener, agentready_server::app())
        .await
        .context("server stopped unexpectedly")
}
