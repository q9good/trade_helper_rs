use super::UpdateAccount;
use crate::market::fund_market::FundData;

#[derive(Debug)]
pub struct FundAccount {
    fund_code: u32,        // 基金代码
    net_value: u32,        // 单位净值
    accumulate_value: u32, //累计净值
    shares: u32,           //持有份额
    cash_bonus: u32,       //现金分红，默认为红利再投
    account_balance: i64,  //账户余额
}

impl UpdateAccount for FundAccount {
    type MarketData = FundData;
    fn update_account(&mut self, data: FundData) {
        if data.dividend.is_some() {
            // 红利再投
            self.cash_bonus += data.dividend.unwrap() * self.shares;
            self.shares += data.dividend.unwrap() * self.shares / data.unit_nav;
        }
        self.net_value = data.unit_nav;
        self.accumulate_value = data.accumulate_nav;
    }
    fn buy(&mut self, data: FundData, volume: u32) {
        self.shares += volume;
        self.account_balance -= (volume * data.unit_nav) as i64;
        self.net_value = data.unit_nav;
        self.accumulate_value = data.accumulate_nav;
    }
    fn sell(&mut self, data: FundData, volume: u32) {
        if volume > self.shares {
            self.shares = 0;
            self.account_balance += (self.shares * data.unit_nav) as i64;
        } else {
            self.shares -= volume;
            self.account_balance += (volume * data.unit_nav) as i64;
        }
        self.net_value = data.unit_nav;
        self.accumulate_value = data.accumulate_nav;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_update_account_without_dividend() {
        let mut account = FundAccount {
            fund_code: 002021,
            net_value: 12880,
            accumulate_value: 22880,
            shares: 1000,
            cash_bonus: 0,
            account_balance: 100000,
        };

        let fund_data = FundData::new(NaiveDate::from_ymd(2021, 9, 30), 20000, 30000, None);
        account.update_account(fund_data);
        println!("{:?}", account);
    }

    #[test]
    fn test_update_account_with_dividend(){
        let mut account = FundAccount {
            fund_code: 002021,
            net_value: 12880,
            accumulate_value: 22880,
            shares: 1000,
            cash_bonus: 0,
            account_balance: 100000,
        };

        let fund_data = FundData::new(NaiveDate::from_ymd(2021, 9, 30), 20000, 30000, Some(100));
        account.update_account(fund_data);
        println!("{:?}", account);
    }

    #[test]
    fn test_account_after_buy(){
        let mut account = FundAccount {
            fund_code: 002021,
            net_value: 12880,
            accumulate_value: 22880,
            shares: 1000,
            cash_bonus: 0,
            account_balance: 100000,
        };

        let fund_data = FundData::new(NaiveDate::from_ymd(2021, 9, 30), 20000, 30000, Some(100));
        account.buy(fund_data,100);
        println!("{:?}", account);
    }

    #[test]
    fn test_account_after_sell(){
        let mut account = FundAccount {
            fund_code: 002021,
            net_value: 12880,
            accumulate_value: 22880,
            shares: 1000,
            cash_bonus: 0,
            account_balance: 100000,
        };

        let fund_data = FundData::new(NaiveDate::from_ymd(2021, 9, 30), 20000, 30000, Some(100));
        account.sell(fund_data,100);
        println!("{:?}", account);
    }
}
