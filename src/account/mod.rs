pub mod fund_account;
pub mod stock_account;
use std::collections::HashMap;

use chrono::NaiveDateTime;
use serde::de;

use crate::market::MarketInfo;

/// 所有账户实现的方法，变更账户信息
pub trait UpdateAccountItem {
    type MarketData: MarketInfo;
    /// 默认账户
    // fn default() -> Self;
    /// 根据行情更新当前持仓信息
    fn update_account(&mut self, data: &Self::MarketData);
    /// 获取当前持仓数量
    fn get_current_volume(&self) -> f32;
    /// 获取当前持仓单价
    fn get_current_value(&self) -> f32;
    /// 获取当前资产
    fn get_current_asset(&self) -> f32;
    /// 以指定数量买入（适用于股票）,返回交易信息
    fn buy_with_volume(&mut self, data: &Self::MarketData, volume: f32) -> TradeDetail;
    /// 以总价买入（适用于基金），返回交易信息
    fn buy_with_cost(&mut self, data: &Self::MarketData, price: f32) -> TradeDetail;
    /// 以指定数量卖出，返回交易信息
    fn sell_with_volume(&mut self, data: &Self::MarketData, volume: f32) -> TradeDetail;
    /// 以持仓比例卖出，返回交易信息
    fn sell_with_proportion(&mut self, data: &Self::MarketData, proportion: f32) -> TradeDetail;
}
/// 交易信息
#[derive(Debug, PartialEq, PartialOrd)]
pub struct TradeItem {
    // 成交价格
    deal_price: f32,
    // 成交数量
    deal_volume: f32,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum TradeDetail {
    Buy(TradeItem),
    Sell(TradeItem),
}

impl TradeDetail {
    //Todo :考虑手续费
    fn calc_cost_or_earning(&self) -> f32 {
        match self {
            Self::Buy(detail) => detail.deal_price * detail.deal_volume * -1.0,
            Self::Sell(detail) => detail.deal_price * detail.deal_volume,
        }
    }
}
/// 交易记录
#[derive(Debug, PartialEq, PartialOrd)]
pub struct TradeHistory {
    // 成交时间
    trade_time: NaiveDateTime,
    // 成交标的代码
    trade_obj: u32,
    // 成交详情
    trade_detail: TradeDetail,
}

/// ## 账户详情
/// ----
///
#[derive(Debug)]
pub struct Account<T: UpdateAccountItem> {
    // 持仓详情
    hold_detail: HashMap<u32, T>,
    // 交易记录
    trade_history: Vec<TradeHistory>,
    // 账面价值
    account_value: f32,
    // 账户余额
    balance_price: f32,
}

impl<T> Account<T>
where
    T: UpdateAccountItem + Default,
{
    /// 新建账户
    fn new() -> Self {
        Account {
            hold_detail: HashMap::<u32, T>::new(),
            trade_history: Vec::<TradeHistory>::new(),
            account_value: 0.0,
            balance_price: 0.0,
        }
    }
    /// 获取持仓单价
    fn get_object_price(&self, code: u32) -> Option<f32> {
        self.hold_detail.get(&code).map(|x| x.get_current_value())
        // .map_or(None, |k| Some(k.get_current_value()))
    }

    /// 获取持仓数量
    fn get_object_volume(&self, code: u32) -> Option<f32> {
        self.hold_detail.get(&code).map(|x| x.get_current_volume())
        // .map_or(None, |k| Some(k.get_current_volume()))
    }

    /// 获取持仓资产总价
    fn get_object_assets(&self, code: u32) -> Option<f32> {
        self.hold_detail.get(&code).map(|x| x.get_current_asset())
        // .map_or(None, |k| Some(k.get_current_asset()))
    }
    // 以指定数量标的买入
    fn buy_with_volume(&mut self, code: u32, info: T::MarketData, volume: f32) {
        let item = self.hold_detail.entry(code).or_insert_with(T::default);
        let prev_asset = item.get_current_asset();
        let detail = item.buy_with_volume(&info, volume);
        let cur_asset = item.get_current_asset();
        // 更新账户余额
        self.balance_price += detail.calc_cost_or_earning();
        // 更新账户资产
        self.account_value = self.account_value - prev_asset + cur_asset;
        // 记录交易信息
        self.trade_history.push(TradeHistory {
            trade_time: info.get_time(),
            trade_obj: code,
            trade_detail: detail,
        });
    }

    // 以指定总价买入
    fn buy_with_cost(&mut self, code: u32, info: T::MarketData, price: f32) {
        let item = self.hold_detail.entry(code).or_insert_with(T::default);
        let prev_asset = item.get_current_asset();
        let detail = item.buy_with_cost(&info, price);
        let cur_asset = item.get_current_asset();
        // 更新账户余额
        self.balance_price += detail.calc_cost_or_earning();
        // 更新账户资产
        self.account_value = self.account_value - prev_asset + cur_asset;
        // 记录交易信息
        self.trade_history.push(TradeHistory {
            trade_time: info.get_time(),
            trade_obj: code,
            trade_detail: detail,
        });
    }

    // 以当前价格卖出指定数量
    fn sell_with_volume(&mut self, code: u32, info: T::MarketData, volume: f32) {
        if let Some(item) = self.hold_detail.get_mut(&code) {
            let prev_asset = item.get_current_asset();
            let detail = item.sell_with_volume(&info, volume);
            let cur_asset = item.get_current_asset();
            // 更新账户余额
            self.balance_price += detail.calc_cost_or_earning();
            // 更新账户资产
            self.account_value = self.account_value - prev_asset + cur_asset;
            // 记录交易信息
            self.trade_history.push(TradeHistory {
                trade_time: info.get_time(),
                trade_obj: code,
                trade_detail: detail,
            });
            // 检查是否全部卖出
            if item.get_current_volume() == 0.0 {
                self.hold_detail.remove_entry(&code);
            }
        }
    }

    // 以持仓比例卖出
    fn sell_with_proportion(&mut self, code: u32, info: T::MarketData, proportion: f32) {
        if let Some(item) = self.hold_detail.get_mut(&code) {
            let prev_asset = item.get_current_asset();
            let detail = item.sell_with_proportion(&info, proportion);
            let cur_asset = item.get_current_asset();
            // 更新账户余额
            self.balance_price += detail.calc_cost_or_earning();
            // 更新账户资产
            self.account_value = self.account_value - prev_asset + cur_asset;
            // 记录交易信息
            self.trade_history.push(TradeHistory {
                trade_time: info.get_time(),
                trade_obj: code,
                trade_detail: detail,
            });
            // 检查是否全部卖出
            if (proportion - 1.0).abs() < 0.0001 {
                self.hold_detail.remove_entry(&code);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use chrono::NaiveDate;

    use crate::market::fund_market::FundData;

    use super::{fund_account::FundAccount, *};

    /// ### 基金账户测试
    /// ----
    /// 只测试以指定价格买入
    #[test]
    fn test_get_ops_when_account_is_empty() {
        let account = Account::<FundAccount>::new();
        assert_eq!(None, account.get_object_volume(000001));
        assert_eq!(None, account.get_object_price(000001));
        assert_eq!(None, account.get_object_assets(000001));
    }

    #[test]
    fn test_buy_new_fund_with_price() {
        let mut account = Account::<FundAccount>::new();
        let fund_data = FundData::new(NaiveDate::from_ymd(2021, 9, 30), 20000, 30000, None);
        let expect_hold_detail = FundAccount {
            net_value: fund_data.unit_nav,
            accumulate_value: fund_data.accumulate_nav,
            shares: 5000,
            cash_bonus: 0,
            total_value: 100000000,
        };
        let expect_trade_history = TradeHistory {
            trade_time: fund_data.date.and_hms(19, 0, 0),
            trade_obj: 1,
            trade_detail: TradeDetail::Buy(TradeItem {
                deal_price: 2.0,
                deal_volume: 50.0,
            }),
        };

        account.buy_with_cost(000001, fund_data, 100.0);

        assert!(account.hold_detail.get(&000001).is_some());
        assert!(account.hold_detail.get(&000002).is_none());
        assert_eq!(expect_hold_detail, account.hold_detail[&000001]);
        assert_eq!(expect_trade_history, account.trade_history.pop().unwrap());
    }

    #[test]
    fn test_buy_same_fund_twice(){
        let mut account = Account::<FundAccount>::new();
        let fund_data1 = FundData::new(NaiveDate::from_ymd(2021, 9, 30), 20000, 30000, None);
        let fund_data2 = FundData::new(NaiveDate::from_ymd(2021, 10, 1), 20000, 30000, None);
        let expect_hold_detail = FundAccount {
            net_value: fund_data2.unit_nav,
            accumulate_value: fund_data2.accumulate_nav,
            shares: 10000,
            cash_bonus: 0,
            total_value: 200000000,
        };
        let expect_trade_history = TradeHistory {
            trade_time: fund_data2.date.and_hms(19, 0, 0),
            trade_obj: 1,
            trade_detail: TradeDetail::Buy(TradeItem {
                deal_price: 2.0,
                deal_volume: 50.0,
            }),
        };

        account.buy_with_cost(000001, fund_data1, 100.0);
        account.buy_with_cost(000001, fund_data2, 100.0);

        assert!(account.hold_detail.get(&000001).is_some());
        assert!(account.hold_detail.get(&000002).is_none());
        assert_eq!(expect_hold_detail, account.hold_detail[&000001]);
        assert_eq!(expect_trade_history, account.trade_history.pop().unwrap());
    }

    #[test]
    fn test_buy_two_different_funds(){
        let mut account = Account::<FundAccount>::new();
        let fund_data1 = FundData::new(NaiveDate::from_ymd(2021, 9, 30), 20000, 30000, None);
        let fund_data2 = FundData::new(NaiveDate::from_ymd(2021, 10, 1), 20000, 30000, None);
        let expect_hold_detail = FundAccount {
            net_value: fund_data2.unit_nav,
            accumulate_value: fund_data2.accumulate_nav,
            shares: 5000,
            cash_bonus: 0,
            total_value: 100000000,
        };
        let expect_trade_history = TradeHistory {
            trade_time: fund_data2.date.and_hms(19, 0, 0),
            trade_obj: 2,
            trade_detail: TradeDetail::Buy(TradeItem {
                deal_price: 2.0,
                deal_volume: 50.0,
            }),
        };

        account.buy_with_cost(000001, fund_data1, 100.0);
        account.buy_with_cost(000002, fund_data2, 100.0);

        assert!(account.hold_detail.get(&000001).is_some());
        assert!(account.hold_detail.get(&000002).is_some());
        assert_eq!(expect_hold_detail, account.hold_detail[&000002]);
        assert_eq!(expect_trade_history, account.trade_history.pop().unwrap());
    }

    #[test]
    fn test_get_ops_with_valid_account(){
        let mut account = Account::<FundAccount>::new();
        let fund_data1 = FundData::new(NaiveDate::from_ymd(2021, 9, 30), 20000, 30000, None);
        let fund_data2 = FundData::new(NaiveDate::from_ymd(2021, 10, 1), 20000, 30000, None);

        account.buy_with_cost(000001, fund_data1, 100.0);
        account.buy_with_cost(000002, fund_data2, 100.0);

        assert_eq!(Some(50.0), account.get_object_volume(000001));
        assert_eq!(Some(2.0), account.get_object_price(000001));
        assert_eq!(Some(100.0), account.get_object_assets(000001));
        assert_eq!(Some(50.0), account.get_object_volume(000002));
        assert_eq!(Some(2.0), account.get_object_price(000002));
        assert_eq!(Some(100.0), account.get_object_assets(000002));
    }
}
