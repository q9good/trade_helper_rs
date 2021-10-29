use anyhow::{anyhow, Result};
use chrono::{NaiveDate, NaiveDateTime};
use itertools::Itertools;
use std::iter::Iterator;

pub mod fund_market;

/// 市场行情
pub trait QuantitativeMarket {
    /// 行情的日期时间
    fn get_info_datetime(&self) -> NaiveDateTime;

    fn query_history_info(code: u32, start_date: NaiveDate, end_date: NaiveDate) -> Vec<Self>
    where
        Self: std::marker::Sized;
}

type MarketCode = u32;

pub trait QueryMarketInfo {
    type MarketInfo: QuantitativeMarket;

    fn query_history_info(
        &self,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<Self::MarketInfo>>;
}

#[derive(Debug)]
pub struct InfoMixer<T: QuantitativeMarket> {
    pub(crate) code: Vec<u32>,
    pub(crate) info: Vec<Vec<T>>,
}

impl<T> InfoMixer<T>
where
    T: QuantitativeMarket,
{
    fn new(codes: &[u32], start_date: NaiveDate, end_date: NaiveDate) -> Self {
        let infos = codes
            .iter()
            .map(|x| T::query_history_info(*x, start_date, end_date))
            // .filter(|x|!x.is_empty())
            .collect();
        InfoMixer {
            code: codes.into(),
            info: infos,
        }
    }
}

impl<T> Iterator for InfoMixer<T>
where
    T: QuantitativeMarket,
{
    type Item = (u32, T);
    fn next(&mut self) -> Option<Self::Item> {
        let future_time = NaiveDate::from_ymd(2099, 1, 1).and_hms(0, 0, 0);
        let first_ele_time: Vec<_> = self
            .info
            .iter()
            .map(|x| {
                let t = x.get(0);

                if let Some(s) = t {
                    s.get_info_datetime()
                } else {
                    future_time
                }
            })
            .collect();
        let valid_time = first_ele_time
            .iter()
            .filter(|x| **x != future_time)
            .collect::<Vec<_>>();
        if valid_time.is_empty() {
            return None;
        }

        let earliest = first_ele_time.iter().min().unwrap();
        let position = first_ele_time.iter().position(|&x| x == *earliest).unwrap();
        let market_data = self.info[position].remove(0);
        Some((self.code[position], market_data))
    }
}
