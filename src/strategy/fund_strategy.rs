use crate::account::fund_account::FundAccount;
use crate::account::{Account, UpdateAccountItem};
use crate::market::fund_market::{get_fund_history, FundData};
use crate::market::InfoMixer;
use std::collections::HashMap;
use time::{macros::*, Date, Duration, Month, OffsetDateTime};

///  Automatic Investment Plan
pub fn run_fund_aip_strategy(
    start: Date,
    end: Date,
    day: u8,
    fund: &[u32],
    budget: &[f32],
) -> Account<FundAccount> {
    let mut fund_mixer = InfoMixer::<FundData>::new(fund, start, end);
    let mut fund_accounts = Account::<FundAccount>::new();
    // let mut prev_fund_month = HashMap::<u32, Month>::new();
    let fund_budget: HashMap<_, _> = fund.iter().zip(budget.iter()).collect();

    let mut prev_fund_month: HashMap<u32, Month> = fund
        .iter()
        .map(|x| (*x, start.month().previous()))
        .collect();
    // Keep the same with real world, won't use statistical way
    fund_mixer.for_each(|(code, fund_data)| {
        if fund_data.date.day() >= day
            && fund_data.date.month() != *prev_fund_month.get(&code).unwrap()
        {
            fund_accounts.buy_with_cost(code, &fund_data, *fund_budget[&code]);
            let entry = prev_fund_month.get_mut(&code).unwrap();
            *entry = fund_data.date.month();
        } else {
            fund_accounts.update_account(code, fund_data);
        }
    });
    let cur_price:u64 = fund_accounts.hold_detail.values().map(|x| x.total_value).sum();
    fund_accounts.account_value = (cur_price as f64 /1000000.0) as f32;
    fund_accounts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_prev_month() {
        let date = date!(2021 - 1 - 1);
        let prev_month = date.month().previous();
        assert_eq!(prev_month,  time::Month::December)
    }

    #[test]
    fn test_single_aip_002021() {
        let start_date = date!(2010 - 1 - 1);
        let end_date = date!(2021 - 1 - 1);
        let result = run_fund_aip_strategy(start_date, end_date, 1, &[002021u32], &[100.0]);
        assert!((result.balance_price + 13200.0).abs()<2.0);
        assert!((result.account_value - 33706.85).abs()<2.0);
    }

    #[test]
    fn  test_single_aip_007994() {
        let start_date = date!(2010 - 1 - 1);
        let end_date = date!(2021 - 1 - 1);
        let result = run_fund_aip_strategy(start_date, end_date, 1, &[007994u32], &[100.0]);
        assert!((result.balance_price + 1000.0).abs()<2.0);
        assert!((result.account_value - 1165.89).abs()<2.0);
    }

    #[test]
    fn  test_single_aip_070032() {
        let start_date = date!(2010 - 1 - 1);
        let end_date = date!(2021 - 1 - 1);
        let result = run_fund_aip_strategy(start_date, end_date, 1, &[070032u32], &[100.0]);
        assert!((result.balance_price + 10300.0).abs()<2.0);
        assert!((result.account_value - 35871.4).abs()<2.0);
    }

    #[test]
    fn  test_single_aip_001875() {
        let start_date = date!(2010 - 1 - 1);
        let end_date = date!(2021 - 1 - 1);
        let result = run_fund_aip_strategy(start_date, end_date, 1, &[001875u32], &[100.0]);
        assert!((result.balance_price + 5700.0).abs()<2.0);
        assert!((result.account_value - 15580.94).abs()<2.0);
    }

    #[test]
    fn test_double_aip(){
        let start_date = date!(2020 - 1 - 1);
        let end_date = date!(2021 - 1 - 1);
        let result = run_fund_aip_strategy(start_date, end_date, 1, &[007994u32, 001875u32], &[100.0, 100.0]);
        assert!((result.balance_price + 2200.0).abs()<2.0);
        assert!((result.account_value - 2926.58).abs()<2.0);
    }

    #[test]
    fn test_triple_aip(){
        let start_date = date!(2020 - 1 - 1);
        let end_date = date!(2021 - 1 - 1);
        let result = run_fund_aip_strategy(start_date, end_date, 1, &[007994u32, 001875u32, 070032u32], &[100.0, 100.0, 100.0]);
        assert!((result.balance_price + 3400.0).abs()<2.0);
        assert!((result.account_value - 4703.69).abs()<2.0);
    }
}
