mod config;
mod db;
mod models;

#[tokio::main]
async fn main() {
    let db_cfg = config::DbConfig::from_env();
    let _client = db::connect_db(&db_cfg).await;
    println!("DB connected. Ready to fetch KuCoin data...");
    // TODO: Реализовать сбор данных KuCoin, сериализацию и запись в БД
}
