use std::fmt;

use crate::api::ApiBuilder;

/// List of private methods.
#[derive(Debug, Copy, Clone)]
pub(crate) enum PrivateMethod {
    // Private User Data
    AddExport,
    Balance,
    ClosedOrders,
    ExportStatus,
    Ledgers,
    OpenOrders,
    OpenPositions,
    QueryLedgers,
    QueryOrders,
    QueryTrades,
    RemoveExport,
    RetrieveExport,
    TradeBalance,
    TradeVolume,
    TradesHistory,
    // Private User Trading
    AddOrder,
    CancelOrder,
    // Private User Funding
    DepositAddresses,
    DepositMethods,
    DepositStatus,
    WalletTransfer,
    Withdraw,
    WithdrawCancel,
    WithdrawInfo,
    WithdrawStatus,
}

impl fmt::Display for PrivateMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Get export report.
pub fn retrieve_export() -> ApiBuilder {
    ApiBuilder::private(PrivateMethod::RetrieveExport)
}

/// Wallet Transfer.
pub fn wallet_transfer() -> ApiBuilder {
    ApiBuilder::private(PrivateMethod::WalletTransfer)
}

/// Request withdrawal cancelation.
pub fn withdraw_cancel() -> ApiBuilder {
    ApiBuilder::private(PrivateMethod::WithdrawCancel)
}

/// Get status of recent withdrawals.
pub fn withdraw_status() -> ApiBuilder {
    ApiBuilder::private(PrivateMethod::WithdrawStatus)
}

/// Withdraw funds.
pub fn withdraw() -> ApiBuilder {
    ApiBuilder::private(PrivateMethod::Withdraw)
}

/// Get withdrawal information.
pub fn withdraw_info() -> ApiBuilder {
    ApiBuilder::private(PrivateMethod::WithdrawInfo)
}

/// Get status of recent deposits.
pub fn deposit_status() -> ApiBuilder {
    ApiBuilder::private(PrivateMethod::DepositStatus)
}

/// Get deposit addresses.
pub fn deposit_addresses() -> ApiBuilder {
    ApiBuilder::private(PrivateMethod::DepositAddresses)
}

/// Get deposit methods.
pub fn deposit_methods() -> ApiBuilder {
    ApiBuilder::private(PrivateMethod::DepositMethods)
}

/// Get account balance.
pub fn balance() -> ApiBuilder {
    ApiBuilder::private(PrivateMethod::Balance)
}

/// Get trace balance.
pub fn trade_balance() -> ApiBuilder {
    ApiBuilder::private(PrivateMethod::TradeBalance)
}

/// Get open orders.
pub fn open_orders() -> ApiBuilder {
    ApiBuilder::private(PrivateMethod::OpenOrders)
}

/// Get closed orders.
pub fn closed_orders() -> ApiBuilder {
    ApiBuilder::private(PrivateMethod::ClosedOrders)
}

/// Query ledgers.
pub fn query_ledgers() -> ApiBuilder {
    ApiBuilder::private(PrivateMethod::QueryLedgers)
}

/// Query orders info.
pub fn query_orders() -> ApiBuilder {
    ApiBuilder::private(PrivateMethod::QueryOrders)
}

/// Get trades history.
pub fn trades_history() -> ApiBuilder {
    ApiBuilder::private(PrivateMethod::TradesHistory)
}

/// Query trades info.
pub fn query_trades() -> ApiBuilder {
    ApiBuilder::private(PrivateMethod::QueryTrades)
}

/// Get open positions.
pub fn open_positions() -> ApiBuilder {
    ApiBuilder::private(PrivateMethod::OpenPositions)
}

/// Get ledgers info.
pub fn ledgers() -> ApiBuilder {
    ApiBuilder::private(PrivateMethod::Ledgers)
}

/// Get trade volume.
pub fn trade_volume() -> ApiBuilder {
    ApiBuilder::private(PrivateMethod::TradeVolume)
}

/// Request export report.
pub fn add_export() -> ApiBuilder {
    ApiBuilder::private(PrivateMethod::AddExport)
}

/// Get export statuses.
pub fn export_status() -> ApiBuilder {
    ApiBuilder::private(PrivateMethod::ExportStatus)
}

/// Remove export report.
pub fn remove_export() -> ApiBuilder {
    ApiBuilder::private(PrivateMethod::RemoveExport)
}

/// Add standard order.
pub fn add_order() -> ApiBuilder {
    ApiBuilder::private(PrivateMethod::AddOrder)
}

/// Cancel open order.
pub fn cancel_order() -> ApiBuilder {
    ApiBuilder::private(PrivateMethod::CancelOrder)
}
