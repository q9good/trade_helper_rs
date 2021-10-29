extern crate crossbeam;
extern crate crossbeam_channel;
mod account;
mod event;
mod market;
mod strategy;

use chrono::{Datelike, Local, NaiveDate};
use crossbeam_channel::{bounded, unbounded};

fn main() {
    let funds = vec![002021u32, 005343];
    let start_date = NaiveDate::from_ymd(2007, 1, 1);
    let today = NaiveDate::from_num_days_from_ce(Local::today().num_days_from_ce());
}
