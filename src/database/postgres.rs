use sqlx::PgPool;

use crate::config::Configs;


pub fn postgress_connect(conf:Configs) -> PgPool {
    let url = format!(
        "postgres://{}:{}@{}:{}/{}", 
        conf.postgres_user,
        conf.postgres_pass,
        conf.postgres_addr,
        conf.postgres_port,
        conf.postgres_data
    );

    
}