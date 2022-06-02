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
        "mexc",
        "okx",
        "zb",
        "zbg",
    ];
    for exchange in exchanges {
        let market_types = crypto_market_type::get_market_types(exchange);
        for market_type in market_types {
            println!("{} {}", exchange, market_type);
            if let Ok(mut markets) = crypto_markets::fetch_markets(exchange, market_type) {
                let file_path =
                    Path::new("./data").join(format!("{}.{}.json", exchange, market_type));
                if file_path.exists() {
                    // merge data
                    let text = std::fs::read_to_string(file_path.as_path()).unwrap();
                    let mut existed_markets =
                        serde_json::from_str::<Vec<crypto_markets::Market>>(&text)
                            .unwrap()
                            .into_iter()
                            .filter(|old_market| {
                                !markets
                                    .iter()
                                    .any(|new_market| new_market.symbol == old_market.symbol)
                            })
                            .collect::<Vec<crypto_markets::Market>>();
                    markets.append(&mut existed_markets);
                }
                markets.sort_by_key(|x| x.symbol.clone());
                let f_out = std::fs::OpenOptions::new()
                    .create(true)
                    .write(true)
                    .truncate(true)
                    .open(file_path.as_path())
                    .unwrap();
                let mut buf_writer = std::io::BufWriter::new(f_out);
                for market in markets.iter_mut() {
                    market.info = serde_json::Map::new(); // clear raw info
                }
                writeln!(
                    buf_writer,
                    "{}",
                    serde_json::to_string_pretty(&markets).unwrap()
                )
                .unwrap();
            }
        }
    }
}
