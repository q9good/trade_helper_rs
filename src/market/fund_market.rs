use chrono::prelude::*;
use reqwest::Url;
use serde::{Deserialize, Deserializer};
use reqwest::header::USER_AGENT;

#[derive(Debug, Deserialize)]
pub struct FundData {
    #[serde(deserialize_with = "deserialize_with_date")]
    date: NaiveDate, // 净值日期
    #[serde(deserialize_with = "deserialize_with_net_value")]
    net_value: u32, // 单位净值
    #[serde(deserialize_with = "deserialize_with_net_value")]
    accumulate_value: u32, //累计净值
    #[serde(deserialize_with = "deserialize_with_grow_rate")]
    grow_rate: i32, //日增长率
    #[serde(deserialize_with = "deserialize_with_trade_status")]
    buy_status: bool, //申购状态
    #[serde(deserialize_with = "deserialize_with_trade_status")]
    sell_status: bool, //赎回状态
    #[serde(deserialize_with = "deserialize_with_dividend")]
    dividend: Option<u32>, //分红配送
}

fn deserialize_with_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let mut s: String = Deserialize::deserialize(deserializer)?;
    unimplemented!()
}

fn deserialize_with_net_value<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let mut s: String = Deserialize::deserialize(deserializer)?;
    unimplemented!()
}

fn deserialize_with_grow_rate<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    let mut s: String = Deserialize::deserialize(deserializer)?;
    unimplemented!()
}

fn deserialize_with_trade_status<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let mut s: String = Deserialize::deserialize(deserializer)?;
    unimplemented!()
}
fn deserialize_with_dividend<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
where
    D: Deserializer<'de>,
{
    let mut s: String = Deserialize::deserialize(deserializer)?;
    unimplemented!()
}

// 查询指定日期范围内的基金数据
fn get_fund_history(code: u32, start_date: NaiveDate, end_date: NaiveDate) -> Vec<FundData> {
    let params = [
        ("type", "lsjz"),
        ("code", &format!("{:0>6}", code)),
        ("page", "1"),
        ("sdate", &start_date.format("%Y-%m-%d").to_string()),
        ("edate", &end_date.format("%Y-%m-%d").to_string()),
        ("per", "20"),
    ];
    let url =
        Url::parse_with_params("http://fund.eastmoney.com/f10/F10DataApi.aspx?", &params).unwrap();
    println!("{}", url);
    let resp = reqwest::blocking::get(url).unwrap().text();
    println!("{:?}", resp);
    Vec::new()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_fund_history() {
        let start_time = NaiveDate::from_ymd(2020, 9, 1);
        let end_time = NaiveDate::from_ymd(2020, 9, 30);
        let code = 007994;
        get_fund_history(code, start_time, end_time);
    }
}
