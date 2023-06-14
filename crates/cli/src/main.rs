use std::net::SocketAddr;

use salvo::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let mut listenfd = listenfd::ListenFd::from_env();
    let (addr, listener) = if let Some(listener) = listenfd.take_tcp_listener(0)? {
        (
            listener.local_addr()?,
            tokio::net::TcpListener::from_std(listener.into()).unwrap(),
        )
    } else {
        let addr: SocketAddr = format!(
            "{}:{}",
            std::env::var("HOST").unwrap_or("127.0.0.1".into()),
            std::env::var("PORT").unwrap_or("8080".into())
        )
        .parse()
        .unwrap();
        (addr, tokio::net::TcpListener::bind(addr).await.unwrap())
    };

    tracing::info!("Listening on {}", addr);
    let acceptor = salvo::conn::tcp::TcpAcceptor::try_from(listener).unwrap();

    let router = Router::new()
        .hoop(salvo::caching_headers::CachingHeaders::default()) // CachingHeader must be before Compression.
        .hoop(salvo::compression::Compression::default().force_priority(true));

    Server::new(acceptor).serve(with_spa_router(router)).await;

    Ok(())
}

#[cfg(debug_assertions)]
fn with_spa_router(router: Router) -> Router {
    router.push(
        Router::with_path("<**rest>").handle(salvo::proxy::Proxy::new("http://localhost:5173")),
    )
}

#[cfg(not(debug_assertions))]
fn with_spa_router(router: Router) -> Router {
    #[derive(rust_embed::RustEmbed)]
    #[folder = "../../apps/client/dist"]
    struct Assets;

    router.push(
        Router::with_path("<**rest>")
            .handle(salvo::serve_static::static_embed::<Assets>().fallback("index.html")),
    )
}
