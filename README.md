# crypto-market-metadata

Scrape cryptocurrency markets metadata from exchanges everyday.

## Data Schema

Each `.json` file under the `data` directory is a list of [`Market`](https://github.com/soulmachine/crypto-crawler-rs/blob/main/crypto-markets/src/market.rs#L28), sorted by `symbol` field.
