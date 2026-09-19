#[tokio::main]
async fn main() -> anyhow::Result<()> {
    rchat_server::run_cli().await
}
