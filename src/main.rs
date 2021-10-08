extern crate crossbeam;
extern crate crossbeam_channel;
mod account;
mod market;
mod strategy;
mod event;
use crossbeam_channel::{bounded, unbounded};
use chrono::{NaiveDate, Local, Datelike};

fn main() {
    let funds = vec![002021u32, 005343];
    let start_date = NaiveDate::from_ymd(2007,1,1);
    let today = NaiveDate::from_num_days_from_ce(Local::today().num_days_from_ce());

}
