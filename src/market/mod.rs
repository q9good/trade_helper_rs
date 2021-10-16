use chrono::NaiveDateTime;

pub mod fund_market;

/// 市场行情
pub trait MarketInfo {
    // 获取时间
    fn get_time(&self) -> NaiveDateTime;
}
