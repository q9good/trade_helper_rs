use super::UpdateAccountItem;
use crate::market::fund_market::FundData;

/// 基金账户信息，为避免浮点运算，部分值乘以一定倍数。
/// 对外接口使用真实值
#[derive(Debug)]
pub struct FundAccount {
    // fund_code: u32,        // 基金代码
    // 净值是其真实价值乘以10000,避免浮点数运算
    pub(crate) net_value: u32, // 单位净值
    accumulate_value: u32,     //累计净值
    // 持有份额是其真实值乘以100,避免浮点数运算
    pub(crate) shares: u32, //持有份额
    cash_bonus: u32,        //现金分红，默认为红利再投
    // 账面价值是真实值乘以1000000
    pub(crate) total_value: u64, //基金总价值
}

impl FundAccount {
    pub(crate) fn new() -> Self {
        FundAccount {
            net_value: 0,
            accumulate_value: 0,
            shares: 0,
            cash_bonus: 0,
            total_value: 0,
        }
    }
}

impl UpdateAccountItem for FundAccount {
    type MarketData = FundData;
    fn update_account(&mut self, data: &FundData) -> f32 {
        if data.dividend.is_some() {
            // 红利再投
            self.cash_bonus += data.dividend.unwrap() * self.shares;
            self.shares += data.dividend.unwrap() * self.shares / data.unit_nav;
        }
        self.net_value = data.unit_nav;
        self.accumulate_value = data.accumulate_nav;
        let prev_value = self.total_value;
        self.total_value = (self.net_value * self.shares) as u64;
        if prev_value > self.total_value {
            ((prev_value - self.total_value) / 1000000) as f32 * -1.0
        } else {
            ((self.total_value - prev_value) / 1000000) as f32
        }
    }
    fn buy_with_volume(&mut self, data: &FundData, volume: f32) -> f32 {
        self.shares += (volume * 100.0) as u32;
        self.net_value = data.unit_nav;
        self.accumulate_value = data.accumulate_nav;
        let prev_value = self.total_value;
        self.total_value = (self.net_value * self.shares) as u64;
        if prev_value > self.total_value {
            ((prev_value - self.total_value) / 1000000) as f32 * -1.0
        } else {
            ((self.total_value - prev_value) / 1000000) as f32
        }
    }
    fn buy_with_cost(&mut self, data: &Self::MarketData, price: f32) -> f32 {
        self.shares += (price * 100.0 / (data.unit_nav as f32) / 10000.0) as u32;
        self.net_value = data.unit_nav;
        self.accumulate_value = data.accumulate_nav;
        let prev_value = self.total_value;
        self.total_value = (self.net_value * self.shares) as u64;
        if prev_value > self.total_value {
            ((prev_value - self.total_value) / 1000000) as f32 * -1.0
        } else {
            ((self.total_value - prev_value) / 1000000) as f32
        }
    }
    fn sell_with_volume(&mut self, data: &FundData, volume: f32) -> f32 {
        let volume = (volume * 100.0) as u32;
        if volume > self.shares {
            self.shares = 0;
        } else {
            self.shares -= volume;
        }
        self.net_value = data.unit_nav;
        self.accumulate_value = data.accumulate_nav;
        let prev_value = self.total_value;
        self.total_value = (self.net_value * self.shares) as u64;
        if prev_value > self.total_value {
            ((prev_value - self.total_value) / 1000000) as f32 * -1.0
        } else {
            ((self.total_value - prev_value) / 1000000) as f32
        }
    }

    fn sell_with_proportion(&mut self, data: &FundData, proportion: f32) -> f32 {
        self.net_value = data.unit_nav;
        self.accumulate_value = data.accumulate_nav;
        match proportion {
            f if (0.0..=1.0).contains(&f) => {
                let sell_volume = (self.shares as f32 * proportion) as u32;
                self.shares -= sell_volume;
                let prev_value = self.total_value;
                self.total_value = (self.net_value * self.shares) as u64;
                if prev_value > self.total_value {
                    ((prev_value - self.total_value) / 1000000) as f32 * -1.0
                } else {
                    ((self.total_value - prev_value) / 1000000) as f32
                }
            }
            _ => 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_update_account_without_dividend() {
        let mut account = FundAccount {
            net_value: 12880,
            accumulate_value: 22880,
            shares: 10000,
            cash_bonus: 0,
            total_value: 128800000,
        };

        let fund_data = FundData::new(NaiveDate::from_ymd(2021, 9, 30), 20000, 30000, None);
        let diff = account.update_account(&fund_data);
        println!("{}-{:?}",diff, account);
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
        println!("{:?}", account);
    }

    #[test]
    fn test_account_after_buy() {
        let mut account = FundAccount {
            // fund_code: 002021,
            net_value: 12880,
            accumulate_value: 22880,
            shares: 10000,
            cash_bonus: 0,
            total_value: 128800000,
        };

        let fund_data = FundData::new(NaiveDate::from_ymd(2021, 9, 30), 20000, 30000, Some(100));
        account.buy_with_volume(&fund_data, 100.0);
        println!("{:?}", account);
    }

    #[test]
    fn test_account_after_sell() {
        let mut account = FundAccount {
            // fund_code: 002021,
            net_value: 12880,
            accumulate_value: 22880,
            shares: 10000,
            cash_bonus: 0,
            total_value: 128800000,
        };

        let fund_data = FundData::new(NaiveDate::from_ymd(2021, 9, 30), 20000, 30000, Some(100));
        account.sell_with_volume(&fund_data, 100.0);
        println!("{:?}", account);
    }
}
