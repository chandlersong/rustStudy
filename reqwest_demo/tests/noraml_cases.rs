#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_binance_ping() {
        let proxy_url = "http://127.0.0.1:7891";
        let client = reqwest::Client::builder()
            .proxy(reqwest::Proxy::all(proxy_url).unwrap())
            .build()
            .unwrap();

        let resp = client.get("https://api.binance.com/api/v3/ping").send().await.unwrap();
        assert!(&resp.status().is_success());
        println!("{:#?}", &resp.text().await.unwrap());
    }


    #[tokio::test]
    async fn test_binance_exchange_info() {
        let proxy_url = "http://127.0.0.1:7891";
        let client = reqwest::Client::builder()
            .proxy(reqwest::Proxy::all(proxy_url).unwrap())
            .build()
            .unwrap();

        let resp = client.get("https://api.binance.com/api/v3/exchangeInfo").send().await.unwrap();
        assert!(&resp.status().is_success());
        println!("{:#?}", &resp.text().await.unwrap());
    }
}
