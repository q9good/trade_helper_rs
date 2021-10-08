pub mod fund_account;
pub mod stock_account;

pub trait UpdateAccount {
    type MarketData;
    // 根据行情更新账户信息
    fn update_account(&mut self, data: &Self::MarketData);
    // 买入
    fn buy_with_volume(&mut self, data: &Self::MarketData, volume: u32);
    // 买入
    fn buy_with_price(&mut self, data: &Self::MarketData, price: u32);
    // 卖出
    fn sell(&mut self, data: &Self::MarketData, volume: u32);
}
