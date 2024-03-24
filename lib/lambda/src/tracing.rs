use tracing::Level;
use tracing_subscriber::fmt;

pub fn init_tracing() {
    fmt()
        .with_max_level(Level::INFO)
        .with_target(false)
        .without_time()
        .init();
}
