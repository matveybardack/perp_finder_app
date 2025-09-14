use tokio_postgres::{Client, NoTls};
use crate::config::DbConfig;

pub async fn connect_db(cfg: &DbConfig) -> Client {
    let conn_str = format!(
        "host={} port={} user={} password={} dbname={}",
        cfg.host, cfg.port, cfg.user, cfg.password, cfg.dbname
    );
    let (client, connection) = tokio_postgres::connect(&conn_str, NoTls).await.expect("DB connect error");
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    client
}
