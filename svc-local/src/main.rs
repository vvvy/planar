use p_local::*;

fn main() {
    let arg = std::env::args().skip(1).next();
    let r = match arg.as_ref().map(|r| r as &str) {
        Some("publish") => publish(),
        Some("consume") => consume(),
        _ => Err(anyhow!("invalid command line"))
    };
    println!("result = {r:?}");
}
