use configuration::Settings;
use server::app;
use server::tracing::init_tracing;
use std::env;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let config_filename: &str = args.get(1).map(|arg| arg.as_str()).unwrap_or("config/application");

    let settings = Settings::load_from_path(config_filename).expect("Failed to read configuration");
    init_tracing(&settings.logging);

    let listener = TcpListener::bind(settings.address())
        .await
        .expect("Failed to bind port");

    println!("{}", BANNER_TEXT);
    tracing::info!("Starting the server...");
    tracing::info!("{}", serde_json::to_string(&settings).unwrap());
    app::run(listener, &settings).await;
}

const BANNER_TEXT: &str = r"
 _                        _         
| |                      | |        
| |_ _ __ ___ _ __   __ _| | _____  
| __| '__/ _ \ '_ \ / _` | |/ / _ \ 
| |_| | |  __/ | | | (_| |   < (_) |
 \__|_|  \___|_| |_|\__,_|_|\_\___/
";
