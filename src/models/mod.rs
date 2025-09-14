// Структуры для таблиц exchanges, markets, tokens и др.

#[derive(Debug)]
pub struct Exchange {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub status: String,
}

#[derive(Debug)]
pub struct Market {
    pub id: i32,
    pub exchange_id: i32,
    pub symbol_raw: String,
    pub base_symbol: String,
    pub quote_symbol: String,
    pub contract_type: String,
    pub is_tradable: bool,
    pub price_step: Option<f64>,
    pub qty_step: Option<f64>,
    pub min_order_notional: Option<f64>,
    pub max_order_notional: Option<f64>,
    pub max_leverage: Option<f64>,
    pub funding_rate: Option<f64>,
    pub status_note: Option<String>,
}

#[derive(Debug)]
pub struct Token {
    pub id: i32,
    pub slug: String,
    pub name: String,
    pub primary_symbol: String,
    pub ambiguous: bool,
}
