use crate::market::fund_market::get_fund_history;
use chrono::{NaiveDate, Local, Datelike, Duration};
use crate::account::fund_account::FundAccount;
use crate::account::UpdateAccount;
use std::collections::HashMap;

///  Automatic Investment Plan
fn run_fund_aip_strategy(fund:u32)->HashMap<u32, FundAccount>{
    let start_date = NaiveDate::from_ymd(2007,1,1);
    let today = NaiveDate::from_num_days_from_ce(Local::today().num_days_from_ce());
    let fund_history = get_fund_history(fund, start_date, today);
    let mut accounts = HashMap::new();
    accounts.insert(fund, FundAccount::new());
    let mut fund_account = accounts.get_mut(&fund).unwrap();
    let mut trade_date;
    if let Ok(events) = fund_history{
        let mut event_iter = events.iter();
        let event = event_iter.next().unwrap();
        fund_account.buy_with_price(event, 1000);
        trade_date = event.date;
        for event in event_iter{
            if event.date - trade_date > Duration::days(30){
                fund_account.buy_with_price(event, 1000);
                trade_date = event.date;
            }else{
                fund_account.update_account(event);
            }
        }
    }
    accounts
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_fund_aip(){
        let fund = 011136u32;
        let account = run_fund_aip_strategy(fund);
        println!("{:?}", account);
        let earnings = (account[&fund].net_value * account[&fund].shares) as f32/10000.0/account[&fund].account_balance as f32;
        println!("grow {:?}", earnings*-1.0);
    }
}