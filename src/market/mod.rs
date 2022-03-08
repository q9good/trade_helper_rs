use anyhow::Result;
use std::fmt::Debug;
use std::iter::Iterator;
use std::sync::Arc;
use time::{macros::*, Date, PrimitiveDateTime};
use reqwest::Client;
use tokio::runtime::Builder;
use tokio::sync::Mutex;
use async_trait::async_trait;

pub mod fund_market;

/// 市场行情
#[async_trait]
pub trait QuantitativeMarket {
    /// 行情的日期时间
    fn get_info_datetime(&self) -> PrimitiveDateTime;

    async fn query_history_info(code: u32, start_date: Date, end_date: Date, cli: Client) -> Vec<Self>
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
    T: QuantitativeMarket + Debug,
{
    pub(crate) fn new(codes: &[u32], start_date: Date, end_date: Date) -> Self {
        let runtime = Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .unwrap();
        let mut mutex_infos = Arc::new(Mutex::new(Vec::<Vec<T>>::new()));
        let mut handles = Vec::with_capacity(codes.len());
        for i in 0..codes.len() {
            handles.push(runtime.spawn(async move {
                let mut client = reqwest::Client::new();
                let ret = T::query_history_info(codes[i], start_date, end_date, client).await;
                let mut copy = mutex_infos.lock().await;
                copy.push(ret);
            }));
        }
        for handle in handles {
            runtime.block_on(handle).unwrap();
        }
        let infos:Vec<_> = Arc::try_unwrap(mutex_infos).unwrap().into_inner();
        // get the inner value of Arc<Mutex<T>>
        // get the inner of MutexGuard<T>
        // let infos:Vec<Vec<T>> =  mutex_infos.into_iter();
        // lock().get_mut();
        // let infos: Vec<Vec<T>> = codes
        //     .iter()
        //     .map(|x| T::query_history_info(*x, start_date, end_date, client))
        //     // .filter(|x|!x.is_empty())
        //     .collect();
        #[cfg(test)]
        println!("{:#?}", infos[0]);
        // infos[0].iter().for_each(|x|println!("{:#?}", x.date));
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

    #[test]
    fn test_call_iter() {
        let start_date = date!(2021 - 10 - 1);
        let end_date = date!(2021 - 10 - 25);
        let codes = [013606_u32, 481010];
        let fund_mixer = InfoMixer::<FundData>::new(&codes, start_date, end_date);
        // let fund_iter = fund_mixer.iter();
    }
}
