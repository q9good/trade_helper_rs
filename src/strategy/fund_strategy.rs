use crate::account::fund_account::FundAccount;
use crate::account::{Account, UpdateAccountItem};
use crate::market::fund_market::{get_fund_history, FundData};
use crate::market::InfoMixer;
use std::collections::HashMap;
use time::{macros::*, Date, Duration, OffsetDateTime};

///  Automatic Investment Plan
pub fn run_fund_aip_strategy(
    start: Date,
    end: Date,
    day: u8,
    fund: &[u32],
    budget: &[f32],
) -> Account<FundAccount>{
    let mut fund_mixer = InfoMixer::<FundData>::new(fund, start, end);
    let mut fund_accounts = Account::<FundAccount>::new();


    fund_accounts
}

fn process_one_specific_fund(
    info: &FundData,
    history: &mut Option<Date>,
    account: &mut Account<FundAccount>,
) {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_fund_aip() {}
}
