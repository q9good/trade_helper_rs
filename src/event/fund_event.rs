use chrono::NaiveDate;
// use itertools::Zip;
use crate::market::fund_market::{FundData, get_fund_history};
use itertools::multizip;
use  itertools::structs::Zip;
// fn gen_funds_event(funds:vec<u32>, start_date:NaiveDate, end_date:NaiveDate)->Zip<FundData>{
//     unimplemented!()
// }