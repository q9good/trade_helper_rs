extern crate crossbeam;
extern crate crossbeam_channel;
// use crate::time::Date;
mod account;
mod event;
mod market;
mod strategy;
use clap::Parser;
use strategy::fund_strategy::*;
use time::{format_description, macros::*, Date, OffsetDateTime};
#[allow(clippy::zero_prefixed_literal)]
// use chrono::{Datelike, Local, };
// use crossbeam_channel::{bounded, unbounded};

/// A CLI APP with fund AIP
#[derive(Parser, Debug)]
#[clap(author, about, version)]
#[clap(name = "FUND AIP")]
struct Opt {
    /// the first day begin to buy fund
    #[clap(short, long, required = true)]
    start: u32,

    /// the last day stop to buy fund
    #[clap(short, long, required = true)]
    end: u32,

    /// the nth day buying fund in a month, default to first day
    #[clap(short, long, default_value = "1")]
    day: u8,

    /// the list of fund code
    #[clap(
        name = "LIST OF FUND",
        short = 'f',
        long,
        required = true,
        min_values = 1
    )]
    fund: Vec<u32>,

    /// the buying amount of each fund
    #[clap(
        name = "BUDGET OF FUND",
        short = 'b',
        long,
        required = true,
        min_values = 1
    )]
    budget: Vec<f32>,
}

fn main() {
    let opt = Opt::parse();
    let format = format_description::parse("[year][month][day]").unwrap();
    let start_date = Date::parse(&opt.start.to_string(), &format).unwrap();
    let end_date = Date::parse(&opt.end.to_string(), &format).unwrap();

    // params check
    if start_date > end_date {
        panic!(
            "the end date {} should later than start date {}",
            opt.end, opt.start
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
    result.show_transaction_detail();
}
