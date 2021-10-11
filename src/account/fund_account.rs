use super::UpdateAccountItem;
use crate::market::fund_market::FundData;

#[derive(Debug)]
pub struct FundAccount {
    // fund_code: u32,        // 基金代码
    pub(crate) net_value: u32,   // 单位净值
    accumulate_value: u32,       //累计净值
    pub(crate) shares: u32,      //持有份额
    cash_bonus: u32,             //现金分红，默认为红利再投
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
    fn update_account(&mut self, data: &FundData) -> i32 {
        if data.dividend.is_some() {
            // 红利再投
            self.cash_bonus += data.dividend.unwrap() * self.shares;
            self.shares += data.dividend.unwrap() * self.shares / data.unit_nav;
        }
        self.net_value = data.unit_nav;
        self.accumulate_value = data.accumulate_nav;
        let prev_value = self.total_value;
        self.total_value = (self.net_value * self.shares) as u64;
        (self.total_value as i64 - prev_value as i64) as i32
    }
    fn buy_with_volume(&mut self, data: &FundData, volume: u32) -> i32 {
        self.shares += volume;
        self.net_value = data.unit_nav;
        self.accumulate_value = data.accumulate_nav;
        let prev_value = self.total_value;
        self.total_value = (self.net_value * self.shares) as u64;
        (self.total_value as i64 - prev_value as i64) as i32
    }
    fn buy_with_cost(&mut self, data: &Self::MarketData, price: u32) -> i32 {
        self.shares += price * 10000 / data.unit_nav;
        self.net_value = data.unit_nav;
        self.accumulate_value = data.accumulate_nav;
        let prev_value = self.total_value;
        self.total_value = (self.net_value * self.shares) as u64;
        (self.total_value as i64 - prev_value as i64) as i32
    }
    fn sell_with_volume(&mut self, data: &FundData, volume: u32) -> i32 {
        if volume > self.shares {
            self.shares = 0;
        } else {
            self.shares -= volume;
        }
        self.net_value = data.unit_nav;
        self.accumulate_value = data.accumulate_nav;
        let prev_value = self.total_value;
        self.total_value = (self.net_value * self.shares) as u64;
        (self.total_value as i64 - prev_value as i64) as i32
    }

    fn sell_with_proportion(&mut self, data: &FundData, proportion: f32) -> i32 {
        self.net_value = data.unit_nav;
        self.accumulate_value = data.accumulate_nav;
        match proportion {
            f if (0.0..=1.0).contains(&f) => {
                let sell_volume = (self.shares as f32 * proportion) as u32;
                self.shares -= sell_volume;
                let prev_value = self.total_value;
                self.total_value = (self.net_value * self.shares) as u64;
                (self.total_value as i64 - prev_value as i64) as i32
            }
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
}
