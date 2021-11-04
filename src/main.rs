use std::io::prelude::*;
use std::path::Path;

fn main() {
    std::fs::create_dir_all("./data").unwrap();
    let exchanges = [
        "binance",
        "bitfinex",
        "bitget",
        "bithumb",
        "bitmex",
        "bitstamp",
        "bybit",
        "coinbase_pro",
        "deribit",
        "dydx",
        "ftx",
        "gate",
        "huobi",
        "kraken",
        "kucoin",
        "mxc",
        "okex",
        "zbg",
    ];
    for exchange in exchanges {
        let market_types = crypto_market_type::get_market_types(exchange);
        for market_type in market_types {
            println!("{} {}", exchange, market_type);
            if let Ok(mut markets) = crypto_markets::fetch_markets(exchange, market_type) {
                markets.sort_by_key(|x| x.symbol.clone());
                let f_out = std::fs::OpenOptions::new()
                    .create(true)
                    .write(true)
                    .truncate(true)
                    .open(Path::new("./data").join(format!("{}.{}.json", exchange, market_type)))
                    .unwrap();
                let mut buf_writer = std::io::BufWriter::new(f_out);
                for market in markets {
                    writeln!(buf_writer, "{}", serde_json::to_string(&market).unwrap()).unwrap();
                }
            }
        }
    }
}
