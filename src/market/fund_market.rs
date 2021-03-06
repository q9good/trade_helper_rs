#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_variables, unused_mut)
)]
use anyhow::{anyhow, Result};
// use itertools::Itertools;
use reqwest::{Client, Url};
use serde::{de, Deserialize, Deserializer};
// use std::collections::HashMap;
use async_trait::async_trait;
use time::{format_description, macros::*, Date, PrimitiveDateTime};

use super::QuantitativeMarket;

/// fund trade status
pub enum FundStatus {
    BuyAvailable,
    SellAvailable,
    TransForbidden,
}

/// fund information
#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq, PartialOrd, Copy, Clone)]
pub struct FundData {
    #[serde(alias = "FSRQ")]
    #[serde(deserialize_with = "deserialize_with_date")]
    pub(crate) date: Date, // 净值日期
    #[serde(alias = "DWJZ")]
    #[serde(deserialize_with = "deserialize_with_price")]
    pub(crate) unit_nav: u32, // 单位净值
    #[serde(alias = "LJJZ")]
    #[serde(deserialize_with = "deserialize_with_price")]
    pub(crate) accumulate_nav: u32, //累计净值
    #[serde(skip_deserializing)]
    SDATE: Option<()>,
    #[serde(skip_deserializing)]
    ACTUALSYI: (),
    #[serde(skip_deserializing)]
    NAVTYPE: (),
    #[serde(skip_deserializing)]
    JZZZL: (),
    //Todo: deserialize it
    #[serde(skip_deserializing)]
    #[serde(alias = "SGZT")]
    buy_status: (),
    #[serde(skip_deserializing)]
    #[serde(alias = "SHZT")]
    sell_status: (),
    #[serde(deserialize_with = "deserialize_with_dividend")]
    #[serde(alias = "FHFCZ")]
    pub(crate) dividend: Option<u32>, //分红
    #[serde(skip_deserializing)]
    FHFCBZ: (),
    #[serde(skip_deserializing)]
    DTYPE: (),
    #[serde(skip_deserializing)]
    FHSP: (),
}

impl FundData {
    pub(crate) fn new(
        date: Date,
        unit_nav: u32,
        accumulate_nav: u32,
        dividend: Option<u32>,
    ) -> Self {
        FundData {
            date,
            unit_nav,
            accumulate_nav,
            SDATE: None,
            ACTUALSYI: (),
            NAVTYPE: (),
            JZZZL: (),
            buy_status: (),
            sell_status: (),
            dividend,
            FHFCBZ: (),
            DTYPE: (),
            FHSP: (),
        }
    }
}

unsafe impl Send for FundData {}

fn deserialize_with_date<'de, D>(deserializer: D) -> Result<Date, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let format = format_description::parse("[year]-[month]-[day]").unwrap();
    Date::parse(&s, &format).map_err(de::Error::custom)
}

fn deserialize_with_price<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let true_price = s.parse::<f32>();
    match true_price {
        Ok(val) => Ok((val * 10000.0) as u32),
        Err(_) => Err(de::Error::custom(format!("can't parse f32{}", s))),
    }
}
fn deserialize_with_dividend<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    if s.is_empty() {
        return Ok(None);
    }
    let true_price = s.parse::<f32>();
    match true_price {
        Ok(val) => Ok(Some((val * 10000.0) as u32)),
        Err(_) => Err(de::Error::custom(format!("can't parse f32{}", s))),
    }
}

// 查询指定日期范围内的基金数据
pub(crate) fn get_fund_history(
    code: u32,
    start_date: Date,
    end_date: Date,
) -> Result<Vec<FundData>> {
    let client = reqwest::blocking::Client::new();
    let format = format_description::parse("[year]-[month]-[day]")?;
    let start_date_str = start_date.format(&format)?;
    let end_date_str = end_date.format(&format)?;
    let params = [
        ("fundCode", format!("{:0>6}", code)),
        ("pageIndex", "1".to_string()),
        ("pageSize", "65535".to_string()),
        ("startDate", start_date_str),
        ("endDate", end_date_str),
    ];
    // http://api.fund.eastmoney.com/f10/lsjz?callback=jQuery18308885693300284889_1643120391690&fundCode=002021&pageIndex=4&pageSize=20&startDate=&endDate=&_=1643120473795
    let url = Url::parse_with_params(
        "http://api.fund.eastmoney.com/f10/lsjz?callback=jQuery18304038998523093684_1586160530315",
        &params,
    )?;
    println!("{}", url);
    let res = client
        .get(url)
        .header("Referer", "http://fundf10.eastmoney.com/".to_string())
        .send()?;
    let content = res.text()?;
    let begin = content.find('[').unwrap();
    let end = content.find(']').unwrap();
    let all_fund_data: Vec<FundData> = serde_json::from_str(&content[begin..=end])?;
    let ret: Vec<FundData> = all_fund_data
        .into_iter()
        .filter(|x| x.date >= start_date && x.date <= end_date)
        // .rev()
        .collect();
    if ret.is_empty() {
        // Err(anyhow::Error::new(error).context("empty"))
        Err(anyhow!(
            "can't fetch fund data between {} and {}",
            start_date,
            end_date
        ))
    } else {
        Ok(ret)
    }
}

#[async_trait]
impl QuantitativeMarket for FundData {
    fn get_info_datetime(&self) -> PrimitiveDateTime {
        self.date.with_hms(19, 0, 0).unwrap()
    }

    async fn query_history_info(
        code: u32,
        start_date: Date,
        end_date: Date,
        cli: Client,
    ) -> Vec<FundData> {
        let format = format_description::parse("[year]-[month]-[day]").unwrap();
        let start_date_str = start_date
            .format(&format)
            .unwrap_or_else(|_| "2000-01-02".to_string());
        let end_date_str = end_date
            .format(&format)
            .unwrap_or_else(|_| "2000-01-01".to_string());
        let params = [
            ("fundCode", format!("{:0>6}", code)),
            ("pageIndex", "1".to_string()),
            ("pageSize", "65535".to_string()),
            ("startDate", start_date_str),
            ("endDate", end_date_str),
        ];
        if let Ok(url) = Url::parse_with_params(
        "http://api.fund.eastmoney.com/f10/lsjz?callback=jQuery18304038998523093684_1586160530315",
        &params ) {
            #[cfg(test)]
            println!("{}", url);
            if let Ok(res) = cli
                .get(url)
                .header(
                    "Referer",
                    "http://fundf10.eastmoney.com".to_string()
                )
                .send().await {
                if let Ok(content) = res.text().await{
                    let begin = content.find('[').unwrap();
                    let end = content.find(']').unwrap();
                    if let Ok(all_fund_data)= serde_json::from_str::<Vec<FundData>>(&content[begin..=end]) {
                        return all_fund_data
                            .into_iter()
                            .rev()
                            .collect::<Vec<FundData>>()
                    }
                }
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_fund_history() {
        let code = 002021;
        let start_date = date!(2021 - 9 - 1);
        let end_date = date!(2021 - 9 - 1);
        let ret = get_fund_history(code, start_date, end_date);
        let expect = vec![FundData {
            date: date!(2021 - 9 - 1),
            unit_nav: 12880,
            accumulate_nav: 38280,
            SDATE: None,
            ACTUALSYI: (),
            NAVTYPE: (),
            JZZZL: (),
            buy_status: (),
            sell_status: (),
            dividend: None,
            FHFCBZ: (),
            DTYPE: (),
            FHSP: (),
        }];
        assert_eq!(expect, ret.unwrap())
    }

    #[test]
    fn test_deserialize_fund_data() {
        let input = "{\"FSRQ\":\"2021-09-15\",\"DWJZ\":\"1.4640\",\"LJJZ\":\"5.0330\",\"SDATE\":null,\"ACTUALSYI\":\"\",\"NAVTYPE\":\"1\",\"JZZZL\":\"-1.45\",\"SGZT\":\"限制大额申购\",\"SHZT\":\"开放赎回\",\"FHFCZ\":\"0.03\",\"FHFCBZ\":\"0\",\"DTYPE\":null,\"FHSP\":\"每份派现金0.0300元\"}";
        let res = serde_json::from_str::<FundData>(input);
        println!("{:#?}", res);
    }

    #[test]
    fn test_long_time_query() {
        let code = 002021;
        let start_date = date!(2015 - 9 - 1);
        let end_date = date!(2021 - 9 - 1);
        let ret = get_fund_history(code, start_date, end_date);

        assert!(ret.is_ok());
    }
}
