use p_local::*;
use tracing::error;
use tokio::task::JoinError;

fn handle_rv(label: &str, rv: std::result::Result<Result<()>, JoinError>) -> Result<()> {
    let reason = match rv {
        Ok(Ok(())) => "no specific reason".into(),
        Ok(Err(e)) => format!("app-error: {}", e),
        Err(e) => format!("join-error: {}", e)
    };
    error!(message="service loop terminated", label=%label, reason=%reason);
    Err(anyhow!("service loop terminated: label={label}, reason={reason}"))
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let runtime = Runtime::new().await?;

    tokio::select! {
        rv = runtime.run_sqlp(sqlp::get_service()) => handle_rv("sqlp", rv),
        rv = runtime.run_disp(disp::get_service()) => handle_rv("disp", rv),
    }

    /* 
    let arg = std::env::args().skip(1).next();
    let r = match arg.as_deref() {
        Some("publish") => publish(),
        Some("consume") => consume(),
        _ => Err(anyhow!("invalid command line"))
    };
    println!("result = {r:?}");
    */

}
