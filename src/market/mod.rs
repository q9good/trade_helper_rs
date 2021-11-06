use anyhow::Result;
use std::iter::Iterator;
use time::{macros::*, Date, PrimitiveDateTime};

pub mod fund_market;

/// 市场行情
pub trait QuantitativeMarket {
    /// 行情的日期时间
    fn get_info_datetime(&self) -> PrimitiveDateTime;

    fn query_history_info(code: u32, start_date: Date, end_date: Date) -> Vec<Self>
    where
        Self: std::marker::Sized;
}

type MarketCode = u32;

pub trait QueryMarketInfo {
    type MarketInfo: QuantitativeMarket;

    fn query_history_info(&self, start_date: Date, end_date: Date)
        -> Result<Vec<Self::MarketInfo>>;
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
    fn new(codes: &[u32], start_date: Date, end_date: Date) -> Self {
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
        let future_time = date!(2099 - 1 - 1).with_hms(0, 0, 0).unwrap();
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
        if !first_ele_time.iter().any(|x| *x != future_time) {
            return None;
        }

        let earliest = first_ele_time.iter().min().unwrap();
        let position = first_ele_time.iter().position(|&x| x == *earliest).unwrap();
        let market_data = self.info[position].remove(0);
        Some((self.code[position], market_data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::market::fund_market::FundData;

    #[test]
    fn test_new_two_funds() {
        let start_date = date!(2021 - 9 - 1);
        let end_date = date!(2021 - 9 - 7);
        let codes = [002190_u32, 481010];
        let fund_mixer = InfoMixer::<FundData>::new(&codes, start_date, end_date);
        assert_eq!(fund_mixer.code, codes);
        assert_eq!(fund_mixer.info.len(), 2);
        assert_eq!(fund_mixer.info[0].len(), 5);
        assert_eq!(fund_mixer.info[1].len(), 5);
    }

    #[test]
    fn test_two_funds_iter() {
        let start_date = date!(2021 - 9 - 1);
        let end_date = date!(2021 - 9 - 7);
        let codes = [002190_u32, 481010];
        let fund_mixer = InfoMixer::<FundData>::new(&codes, start_date, end_date);
        fund_mixer.for_each(|(code, info)| println!("{:?}: {}", info.date, code));
    }

    #[test]
    fn test_unbalanced_two_funds_iter() {
        let start_date = date!(2021 - 10 - 1);
        let end_date = date!(2021 - 10 - 25);
        let codes = [013606_u32, 481010];
        let fund_mixer = InfoMixer::<FundData>::new(&codes, start_date, end_date);
        fund_mixer.for_each(|(code, info)| println!("{:?}: {}", info.date, code));
    }
}
