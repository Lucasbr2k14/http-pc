use redis::aio::ConnectionManager;

use crate::config::Configs;


pub async fn redis_connect(config: Configs) -> ConnectionManager {
    let url = format!("redis://{}:{}", config.redis_addr, config.redis_port);
    println!("{}", url);
    let client = redis::Client::open(url)
    .expect("Erro ao conectar ao redis");

    let connection = client
    .get_connection_manager()
    .await
    .expect("Erro ao conectar ao redis");

    connection
}
