extern crate crossbeam;
extern crate crossbeam_channel;
// use crate::time::Date;
mod account;
mod event;
mod market;
mod strategy;
use clap::Parser;
use strategy::fund_strategy::*;
use time::{format_description, Date};
#[allow(clippy::zero_prefixed_literal)]
// use crossbeam_channel::{bounded, unbounded};

/// A CLI APP FOR TRADING
#[derive(Parser, Debug)]
#[clap(author, about, version)]
#[clap(name = "TRADER'S HELPER")]
struct Opt {
    /// the first day begin to buy fund
    #[clap(short, long, required = true)]
    begin: u32,

    /// the last day stop to buy fund
    #[clap(short, long, required = true)]
    end: u32,

    /// [optional] the nth day buying fund in a month, default to first day
    #[clap(short, long, default_value = "1")]
    day: u8,

    /// the list of fund code
    #[clap(name = "FUND LIST", short = 'f', long, required = true, min_values = 1)]
    fund: Vec<u32>,

    /// the buying amount of each fund
    #[clap(
        name = "BUDGET PLAN FOR FUNDS",
        short = 'p',
        long,
        required = true,
        min_values = 1
    )]
    budget: Vec<f32>,

    /// [optional] whether show the specific trade detail or not
    #[clap(short, parse(from_flag))]
    specific: bool,
}

fn main() {
    let opt = Opt::parse();
    let format = format_description::parse("[year][month][day]").unwrap();
    let start_date = Date::parse(&opt.begin.to_string(), &format).unwrap();
    let end_date = Date::parse(&opt.end.to_string(), &format).unwrap();

    // params check
    if start_date > end_date {
        panic!(
            "the end date {} should later than start date {}",
            opt.end, opt.begin
        );
    }

    if opt.fund.len() != opt.budget.len() {
        panic!(
            "the length of fund: {:?} and budget: {:?} must match",
            opt.fund, opt.budget
        );
    }

    let result = run_fund_aip_strategy(start_date, end_date, opt.day, &opt.fund, &opt.budget);
    result.show_hold_detail();
    if opt.specific {
        result.show_transaction_detail();
    }
    println!(
        "At last, account value: {value:.2}",
        value = result.account_value as f64 * 0.000001
    );
    println!(
        "currency: {currency:.2}",
        currency = result.balance_price as f64 * 0.000001
    );
}
