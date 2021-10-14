pub mod fund_account;
pub mod stock_account;
use std::collections::HashMap;

use chrono::NaiveDateTime;

use crate::market::MarketInfo;

/// 变更账户信息
pub trait UpdateAccountItem {
    type MarketData;
    // 根据行情更新账户信息
    fn update_account(&mut self, data: &Self::MarketData);
    // 获取当前持仓金额
    fn get_current_value(&self) -> f32;
    // 以数量买入,返回购入费用和账面变更
    fn buy_with_volume(&mut self, data: &Self::MarketData, volume: f32) -> TradeDetail;
    // 以总价买入，返回购入费用和账面变更
    fn buy_with_cost(&mut self, data: &Self::MarketData, price: f32) -> TradeDetail;
    // 以数量卖出，返回卖出价格和账面变更
    fn sell_with_volume(&mut self, data: &Self::MarketData, volume: f32) -> TradeDetail;
    // 以持仓比例卖出，返回卖出价格和账面变更
    fn sell_with_proportion(&mut self, data: &Self::MarketData, proportion: f32) -> TradeDetail;
}
/// 交易信息
#[derive(Debug, PartialEq)]
pub struct TradeItem {
    // 成交价格
    deal_price: f32,
    // 成交数量
    deal_volume: f32,
}

#[derive(Debug, PartialEq)]
pub enum TradeDetail {
    Buy(TradeItem),
    Sell(TradeItem),
}
/// 交易记录
#[derive(Debug)]
pub struct TradeHistory {
    // 成交时间
    trade_time: NaiveDateTime,
    // 成交
    trade_obj: u32,
    // 成交详情
    trade_detail: TradeDetail,
}

#[derive(Debug)]
pub struct Account<T: UpdateAccountItem> {
    // 持仓详情
    holder_detail: HashMap<u32, T>,
    // 交易记录
    trade_history: Vec<TradeHistory>,
    // 账面价值
    account_value: u64,
    // 账户余额
    balance_price: i64,
}

impl<T> Account<T>
where
    T: UpdateAccountItem,
{
    // 获取持仓价格
    fn get_object_price(&self, code: u32) -> Option<f32> {
        unimplemented!()
    }

    // 获取持仓数量
    fn get_object_volume(&self, code: u32) -> Option<u32> {
        unimplemented!()
    }

    // 以指定数量标的买入
    fn buy_with_volume<M: MarketInfo>(&mut self, info: M, volume: u32) {
        unimplemented!()
    }

    // 以指定总价买入
    fn buy_with_cost<M: MarketInfo>(&mut self, info: M, price: f32) {
        unimplemented!()
    }

    // 以当前价格卖出指定数量
    fn sell_with_volume<M: MarketInfo>(&mut self, info: M, volume: u32) {
        unimplemented!()
    }

    // 以持仓比例卖出
    fn sell_with_proportion<M: MarketInfo>(&mut self, info: M, proportion: u32) {}
}
