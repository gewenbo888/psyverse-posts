//! Revenue / cashflow domain model (SovereignPay + RevenueDashboard).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueEvent {
    pub id: String,
    pub source: RevenueSource,
    pub amount: f64,
    pub currency: String,
    pub platform: String,
    pub timestamp: String,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RevenueSource {
    WeChatTip,
    Web3Payment,
    AdRevenue,
    BountyPayout,
    Affiliate,
}
