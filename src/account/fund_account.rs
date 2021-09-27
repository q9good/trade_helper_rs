use super::UpdateAccount;
use crate::market::fund_market::FundData;

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
        unimplemented!()
    }
    fn buy(&mut self, data: FundData, volume: u32) {
        todo!()
    }
    fn sell(&mut self, data: FundData, volume: u32) {
        todo!()
    }
}
