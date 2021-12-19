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
        if fund_data.date.day() > day
            && fund_data.date.month() != *prev_fund_month.get(&code).unwrap()
        {
            #[cfg(test)]
            println!("day: {:?} month: {:?} prev_month:{:?}", fund_data.date.day(), fund_data.date.month(), prev_fund_month.get(&code));
            fund_accounts.buy_with_cost(code, fund_data, *fund_budget[&code]);
            let entry = prev_fund_month.get_mut(&code).unwrap();
            *entry = entry.next();
        } //else{
          // fund_accounts.update_account(code, fund_data);
          //}
    });
    fund_accounts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_prev_month() {
        let date = date!(2021 - 1 - 1);
        let prev_month = date.month().previous();
        println!("{:?}", prev_month)
    }

    #[test]
    fn test_run_aip(){
        let start_date = date!(2021 -1 -1);
        let end_date = date!(2021 -3 -1);
        let result = run_fund_aip_strategy(start_date, end_date, 1, &[002021u32], &[100.0]);
        println!("{:?}", result);
    }
}
