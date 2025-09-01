use serde::Deserialize;

/// 交易所信息结构体
/// 包含交易所的时区、服务器时间、速率限制规则、交易所过滤器和所有交易对的详细信息
#[derive(Deserialize, Debug)]
pub struct ExchangeInfo {
    #[serde(rename = "timezone")]
    /// 时区，通常为 "UTC"
    pub timezone: String,

    #[serde(rename = "serverTime")]
    /// 服务器当前时间戳（毫秒）
    pub server_time: u64,

    #[serde(rename = "rateLimits")]
    /// 速率限制规则数组
    pub rate_limits: Vec<RateLimit>,

    #[serde(rename = "exchangeFilters")]
    /// 交易所级过滤器数组
    pub exchange_filters: Vec<ExchangeFilter>,

    #[serde(rename = "symbols")]
    /// 交易对列表数组
    pub symbols: Vec<ExchangeSymbol>,
}

/// 速率限制规则结构体
/// 定义 API 调用的速率限制，包括类型、间隔和限制值
#[derive(Deserialize, Debug)]
pub struct RateLimit {
    #[serde(rename = "rateLimitType")]
    /// 限制类型，如 "REQUEST_WEIGHT"
    pub rate_limit_type: String,

    #[serde(rename = "interval")]
    /// 时间间隔，如 "MINUTE"
    pub interval: String,

    #[serde(rename = "intervalNum")]
    /// 间隔数量
    pub interval_num: i32,

    #[serde(rename = "limit")]
    /// 限制值
    pub limit: i32,
}

/// 交易所过滤器结构体
/// 定义交易所级别的过滤规则，使用标签区分不同类型
#[derive(Deserialize, Debug)]
#[serde(tag = "filterType")]
pub enum ExchangeFilter {
    #[serde(rename = "PRICE_FILTER")]
    /// 价格过滤器，包含最小/最大价格和价格步长
    PriceFilter {
        #[serde(rename = "minPrice", default)]
        min_price: Option<String>,
        #[serde(rename = "maxPrice", default)]
        max_price: Option<String>,
        #[serde(rename = "tickSize", default)]
        tick_size: Option<String>,
    },
    #[serde(rename = "LOT_SIZE")]
    /// 数量过滤器，包含最小/最大数量和数量步长
    LotSize {
        #[serde(rename = "minQty", default)]
        min_qty: Option<String>,
        #[serde(rename = "maxQty", default)]
        max_qty: Option<String>,
        #[serde(rename = "stepSize", default)]
        step_size: Option<String>,
    },
    #[serde(other)]
    /// 未知过滤器类型
    Unknown,
}
/// 交易对信息结构体
/// 包含单个交易对的详细信息，包括基本信息、状态、交易规则和权限
#[derive(Deserialize, Debug)]
pub struct ExchangeSymbol {
    #[serde(rename = "symbol")]
    /// 交易对符号，如 "BTCUSDT"
    pub symbol: String,
    #[serde(rename = "status")]
    /// 交易状态，可能的值包括：TRADING, END_OF_DAY, HALT, BREAK
    pub status: String,
    #[serde(rename = "baseAsset")]
    /// 基础资产，如 "BTC"
    pub base_asset: String,
    #[serde(rename = "baseAssetPrecision")]
    /// 基础资产精度
    pub base_asset_precision: i32,
    #[serde(rename = "quoteAsset")]
    /// 报价资产，如 "USDT"
    pub quote_asset: String,
    #[serde(rename = "quotePrecision")]
    /// 报价精度（即将废弃）
    pub quote_precision: i32,
    #[serde(rename = "quoteAssetPrecision")]
    /// 报价资产精度
    pub quote_asset_precision: i32,
    #[serde(rename = "baseCommissionPrecision")]
    /// 基础手续费精度
    pub base_commission_precision: i32,
    #[serde(rename = "quoteCommissionPrecision")]
    /// 报价手续费精度
    pub quote_commission_precision: i32,
    #[serde(rename = "orderTypes")]
    /// 支持的订单类型数组，可能的值包括：LIMIT, LIMIT_MAKER, MARKET, STOP_LOSS, STOP_LOSS_LIMIT, TAKE_PROFIT, TAKE_PROFIT_LIMIT
    pub order_types: Vec<String>,
    #[serde(rename = "icebergAllowed")]
    /// 是否允许冰山订单
    pub iceberg_allowed: bool,
    #[serde(rename = "ocoAllowed")]
    /// 是否允许 OCO 订单
    pub oco_allowed: bool,
    #[serde(rename = "quoteOrderQtyMarketAllowed")]
    /// 是否允许按报价数量下市价单
    pub quote_order_qty_market_allowed: bool,
    #[serde(rename = "allowTrailingStop")]
    /// 是否允许追踪止损
    pub allow_trailing_stop: bool,
    #[serde(rename = "cancelReplaceAllowed")]
    /// 是否允许取消并替换
    pub cancel_replace_allowed: bool,
    #[serde(rename = "isSpotTradingAllowed")]
    /// 是否允许现货交易
    pub is_spot_trading_allowed: bool,
    #[serde(rename = "isMarginTradingAllowed")]
    /// 是否允许保证金交易
    pub is_margin_trading_allowed: bool,
    /// 交易对级过滤器数组
    pub filters: Vec<ExchangeFilter>,
    /// 交易对权限数组
    pub permissions: Vec<String>,
    #[serde(rename = "defaultSelfTradePreventionMode")]
    /// 默认自成交预防模式
    pub default_self_trade_prevention_mode: String,
    #[serde(rename = "allowedSelfTradePreventionModes")]
    /// 允许的自成交预防模式数组
    pub allowed_self_trade_prevention_modes: Vec<String>,
}

#[cfg(test)]
mod tests {
    use reqwest::Client;
    use wiremock::{Mock, MockServer, ResponseTemplate};
    use wiremock::matchers::method;
    use super::*;

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
        let info: ExchangeInfo = resp.json().await.unwrap();
        // 断言部分字段，确保结构体内容有效
        assert!(!info.timezone.is_empty());
        assert!(!info.symbols.is_empty());

        println!("历史上有{:#?}交易对", info.symbols.len());

    }

    #[tokio::test]
    async fn test_reqwest_with_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_string("Hello from mock!"))
            .mount(&mock_server)
            .await;

        let client = Client::builder()
            .no_proxy() // 明确禁用所有代理
            .build().unwrap();
        let response = client.get(&mock_server.uri()).send().await.unwrap();
        assert_eq!(response.status(), 200);
        let body = response.text().await.unwrap();
        assert_eq!(body, "Hello from mock!");
    }
}
