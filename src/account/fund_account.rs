use super::{TradeDetail, TradeItem, UpdateAccountItem};
use crate::market::fund_market::FundData;

/// 基金账户信息，为避免浮点运算，部分值乘以一定倍数。
/// 对外接口使用真实值
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct FundAccount {
    // fund_code: u32,        // 基金代码
    // 净值是其真实价值乘以10000,避免浮点数运算
    pub(crate) net_value: u32,        // 单位净值
    pub(crate) accumulate_value: u32, //累计净值
    // 持有份额是其真实值乘以100,避免浮点数运算
    pub(crate) shares: u32,     //持有份额
    pub(crate) cash_bonus: u32, //现金分红，默认为红利再投
    // 账面价值是真实值乘以1000000
    pub(crate) total_value: u64, //基金总价值
}

impl FundAccount {
    fn check_dividend(&mut self, data: &FundData) {
        if data.dividend.is_some() {
            // 红利再投
            self.cash_bonus += data.dividend.unwrap() * self.shares;
            self.shares += data.dividend.unwrap() * self.shares / data.unit_nav;
        }
    }
}

impl UpdateAccountItem for FundAccount {
    type MarketData = FundData;
    // fn default() -> Self {}
    fn update_account(&mut self, data: &FundData) {
        self.check_dividend(data);
        self.net_value = data.unit_nav;
        self.accumulate_value = data.accumulate_nav;
        self.total_value = (self.net_value * self.shares) as u64;
    }

    fn get_current_value(&self) -> f32 {
        self.net_value as f32 * 0.0001
    }
    fn get_current_volume(&self) -> f32 {
        self.shares as f32 * 0.01
    }

    fn get_current_asset(&self) -> f32 {
        (self.total_value as f64 * 0.000001) as f32
    }

    fn buy_with_volume(&mut self, data: &FundData, volume: f32) -> TradeDetail {
        self.check_dividend(data);
        self.shares += (volume * 100.0) as u32;
        self.net_value = data.unit_nav;
        self.accumulate_value = data.accumulate_nav;
        self.total_value = (self.net_value * self.shares) as u64;
        TradeDetail::Buy(TradeItem {
            deal_price: self.net_value as f32 * 0.0001,
            deal_volume: volume,
        })
    }
    fn buy_with_cost(&mut self, data: &Self::MarketData, price: f32) -> TradeDetail {
        self.check_dividend(data);
        let increment = ((price / ((data.unit_nav as f32) * 0.0001)) * 100.0) as u32;
        self.shares += increment;
        self.net_value = data.unit_nav;
        self.accumulate_value = data.accumulate_nav;
        self.total_value = (self.net_value * self.shares) as u64;
        TradeDetail::Buy(TradeItem {
            deal_price: self.net_value as f32 * 0.0001,
            deal_volume: increment as f32 * 0.01,
        })
    }
    fn sell_with_volume(&mut self, data: &FundData, volume: f32) -> TradeDetail {
        // Todo :卖出当天能享受分红否？
        self.check_dividend(data);
        let decrement = if volume < self.shares as f32 * 0.01 {
            (volume * 100.0) as u32
        } else {
            self.shares
        };
        self.shares -= decrement;
        self.net_value = data.unit_nav;
        self.accumulate_value = data.accumulate_nav;
        self.total_value = (self.net_value * self.shares) as u64;
        TradeDetail::Sell(TradeItem {
            deal_price: self.net_value as f32 * 0.0001,
            deal_volume: decrement as f32 * 0.01,
        })
    }

    fn sell_with_proportion(&mut self, data: &FundData, proportion: f32) -> TradeDetail {
        self.check_dividend(data);
        self.net_value = data.unit_nav;
        self.accumulate_value = data.accumulate_nav;
        match proportion {
            f if (0.0..=1.0).contains(&f) => {
                let sell_volume = (self.shares as f32 * proportion) as u32;
                self.shares -= sell_volume;
                self.total_value = (self.net_value * self.shares) as u64;
                TradeDetail::Sell(TradeItem {
                    deal_price: self.net_value as f32 * 0.0001,
                    deal_volume: sell_volume as f32 * 0.01,
                })
            }
            _ => {
                self.total_value = (self.net_value * self.shares) as u64;
                TradeDetail::Sell(TradeItem {
                    deal_price: 0.0,
                    deal_volume: 0.0,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_update_account_without_dividend_rise() {
        let mut account = FundAccount {
            net_value: 12880,
            accumulate_value: 22880,
            shares: 10000,
            cash_bonus: 0,
            total_value: 128800000,
        };

        let fund_data = FundData::new(NaiveDate::from_ymd(2021, 9, 30), 20000, 30000, None);
        account.update_account(&fund_data);
        assert_eq!(account.total_value, 200000000);
    }

    #[test]
    fn test_update_account_without_dividend_fall() {
        let mut account = FundAccount {
            net_value: 12880,
            accumulate_value: 22880,
            shares: 10000,
            cash_bonus: 0,
            total_value: 128800000,
        };

        let fund_data = FundData::new(NaiveDate::from_ymd(2021, 9, 30), 10000, 30000, None);
        account.update_account(&fund_data);
        assert_eq!(account.total_value, 100000000);
    }

    #[test]
    fn test_update_account_with_dividend() {
        let mut account = FundAccount {
            // fund_code: 002021,
            net_value: 12880,
            accumulate_value: 22880,
            shares: 10000,
            cash_bonus: 0,
            total_value: 128800000,
        };

        let fund_data = FundData::new(NaiveDate::from_ymd(2021, 9, 30), 20000, 30000, Some(100));
        account.update_account(&fund_data);
        assert_eq!(account.total_value, 201000000);
    }

    #[test]
    fn test_account_after_buy_volume() {
        let mut account = FundAccount {
            // fund_code: 002021,
            net_value: 12880,
            accumulate_value: 22880,
            shares: 10000,
            cash_bonus: 0,
            total_value: 128800000,
        };

        let expect = TradeDetail::Buy(TradeItem {
            deal_price: 2.0,
            deal_volume: 100.0,
        });

        let fund_data = FundData::new(NaiveDate::from_ymd(2021, 9, 30), 20000, 30000, None);
        let detail = account.buy_with_volume(&fund_data, 100.0);
        assert_eq!(account.shares, 20000);
        assert_eq!(account.total_value, 400000000);
        assert_eq!(detail, expect);
    }

    #[test]
    fn test_account_after_buy_price() {
        let mut account = FundAccount {
            // fund_code: 002021,
            net_value: 12880,
            accumulate_value: 22880,
            shares: 10000,
            cash_bonus: 0,
            total_value: 128800000,
        };
        let expect = TradeDetail::Buy(TradeItem {
            deal_price: 2.0,
            deal_volume: 100.0,
        });

        let fund_data = FundData::new(NaiveDate::from_ymd(2021, 9, 30), 20000, 30000, None);
        let detail = account.buy_with_cost(&fund_data, 200.0);
        assert_eq!(account.shares, 20000);
        assert_eq!(account.total_value, 400000000);
        assert_eq!(detail, expect);
    }

    #[test]
    fn test_account_after_sell_volume() {
        let mut account = FundAccount {
            // fund_code: 002021,
            net_value: 12880,
            accumulate_value: 22880,
            shares: 10000,
            cash_bonus: 0,
            total_value: 128800000,
        };
        let expect = TradeDetail::Sell(TradeItem {
            deal_price: 2.0,
            deal_volume: 50.0,
        });

        let fund_data = FundData::new(NaiveDate::from_ymd(2021, 9, 30), 20000, 30000, None);
        let detail = account.sell_with_volume(&fund_data, 50.0);
        assert_eq!(account.shares, 5000);
        assert_eq!(account.total_value, 100000000);
        assert_eq!(detail, expect);
    }

    #[test]
    fn test_account_after_sell_proportion() {
        let mut account = FundAccount {
            // fund_code: 002021,
            net_value: 12880,
            accumulate_value: 22880,
            shares: 10000,
            cash_bonus: 0,
            total_value: 128800000,
        };
        let expect = TradeDetail::Sell(TradeItem {
            deal_price: 2.0,
            deal_volume: 50.0,
        });

        let fund_data = FundData::new(NaiveDate::from_ymd(2021, 9, 30), 20000, 30000, None);
        let detail = account.sell_with_proportion(&fund_data, 0.5);
        // println!("{:?}", account);
        assert_eq!(account.shares, 5000);
        assert_eq!(account.total_value, 100000000);
        assert_eq!(detail, expect);
    }
}
