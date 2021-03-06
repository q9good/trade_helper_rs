#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_variables, unused_mut)
)]

//! ## 市场信息
//! ----
//!  
//! ### Trait QuantitativeMarket
//! ----
//! 定义市场行情数据的获取方法
//! + get_info_datetime: 获取当前市场行情的时间信息
//! + query_history_info: 获取指定时间范围内某具体投资标的(由code指定)的行情信息
//!
//! ### Struct InfoMixer
//! ----
//! 用于同时关注多个标的的行情信息，实现Iterator接口，将按时间先后顺序返回行情信息
//! + code: 关注标的的代码
//! + info：各个关注标的的行情信息，每个具体标的的行情信息是一个Vec<T: QuantitativeMarket>

use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use std::fmt::Debug;
use std::iter::Iterator;
use std::sync::Arc;
use time::{macros::*, Date, PrimitiveDateTime};
use tokio::runtime::Builder;
use tokio::sync::Mutex;

pub mod fund_market;

/// 市场行情
#[async_trait]
pub trait QuantitativeMarket: Send + Copy + 'static {
    /// 行情的日期时间
    fn get_info_datetime(&self) -> PrimitiveDateTime;

    async fn query_history_info(
        code: u32,
        start_date: Date,
        end_date: Date,
        cli: Client,
    ) -> Vec<Self>
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
        let mutex_infos = Arc::new(Mutex::new(Vec::<(u32, Vec<T>)>::new()));
        let mut handles = Vec::with_capacity(codes.len());
        let owned_codes = codes.to_owned();
        // for i in 0..codes.len() {
        for code in owned_codes.into_iter().take(codes.len()) {
            let info_copy = mutex_infos.clone();
            handles.push(runtime.spawn(async move {
                let client = reqwest::Client::new();
                let ret = T::query_history_info(code, start_date, end_date, client).await;
                let mut copy = info_copy.lock().await;
                copy.push((code, ret));
            }));
        }
        for handle in handles {
            runtime.block_on(handle).unwrap();
        }
        let infos: Vec<_> = Arc::try_unwrap(mutex_infos).unwrap().into_inner();
        // let mut code_infos: Vec<_> = Vec::with_capacity(infos.len());
        let mut code_infos: Vec<Vec<_>> = vec![Vec::new(); codes.len()];
        for (code, infos) in infos {
            let idx = codes.iter().position(|&c| c == code).unwrap();
            code_infos[idx] = infos;
        }

        InfoMixer {
            code: codes.into(),
            info: code_infos,
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
        let codes = [002190, 481010];
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
