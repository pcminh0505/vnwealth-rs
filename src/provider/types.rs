use anyhow::Result;

pub trait DataProvider {
    fn new() -> Self;
    async fn fetch_asset_price(&self, asset_name: Option<String>) -> Result<f32>;
}
