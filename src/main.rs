extern crate crossbeam;
extern crate crossbeam_channel;
// use crate::time::Date;
mod account;
mod event;
mod market;
mod strategy;
use time::{macros::*, OffsetDateTime};
#[allow(clippy::zero_prefixed_literal)]
// use chrono::{Datelike, Local, };
// use crossbeam_channel::{bounded, unbounded};

fn main() {
    let funds = vec![002021u32, 005343];
    let _start_date = date!(2007 - 1 - 1);
    let _today = OffsetDateTime::now_local().unwrap().date();
}
