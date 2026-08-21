use std::env;

#[derive(Clone, Debug)]
pub struct Configs {
    // Configs from app
    pub ip:   String,
    pub port: String,
    
    // Configs from postgress
    pub postgres_addr:  String,
    pub postgres_user:  String,
    pub postgres_pass:  String,
    pub postgres_port:  String,
    pub postgres_data:  String,
    // Configs from redis
    pub redis_addr: String,
    pub redis_user: String,
    pub redis_pass: String,
    pub redis_port: String,

    // Configs from Json Web Token
    pub json_web_token_secret: String,
}

pub fn from_env() -> Configs {
    Configs {
        ip: env::var("IP")
            .expect("Ip not defined from .env"),
        port: env::var("PORT")
            .expect("Port not defined from .env"),
        postgres_addr: env::var("POSTGRES_ADDR")
            .expect("Postgress addres not defined from .env"),
        postgres_user: env::var("POSTGRES_USER")
            .expect("Postgress user not defined from .env"),
        postgres_pass: env::var("POSTGRES_PASS")
            .expect("Postgress user not defined from .env"),
        postgres_port: env::var("POSTGRES_PORT")
            .expect("Postgress port not defined from .env"),
        postgres_data: env::var("POSTGRES_DATA")
            .expect("Postgres database not defined from .env"),
        redis_addr: env::var("REDIS_ADDR")
            .expect("Redis addres not defined from .env"),
        redis_user: env::var("REDIS_USER")
            .expect("Redis user not defined from .env"),
        redis_pass: env::var("REDIS_PASS")
            .expect("Redis password not defined from .env"),
        redis_port: env::var("REDIS_PASS")
            .expect("Redis port not defined from .env"),
        json_web_token_secret: env::var("JWT_SECRET")
            .expect("Json Web Token secret not defined from .env")
    }
}