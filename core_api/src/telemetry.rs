use tracing_subscriber::
{
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};

use crate::config::Config;

pub fn init(config: &Config)
{
    let env_filter = EnvFilter::new(&config.rust_log);

    if config.is_production()
    {
        tracing_subscriber::registry()
            .with(env_filter)
            .with(tracing_subscriber::fmt::layer().json())
            .init();
    }
    else
    {
        tracing_subscriber::registry()
            .with(env_filter)
            .with(tracing_subscriber::fmt::layer().compact())
            .init();
    }
}