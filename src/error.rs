use thiserror::Error;

#[derive(Error, Debug)]
pub enum CryptfolioError {
    // platform errors
    #[error("Platform is already staged for connection")]
    PlatformAlreadyExists,

    // coin errors
    #[error("Coin already exists (code: {0} | name: {1}")]
    CoinAlreadyExists(String, String),

    // database errors
    #[error("Could not establish with database at \"{0}\"")]
    DatabaseConnectionFailed(String),
    
    #[error("Database Query Failure: {0}")]
    DatabaseQueryFailed(String),

    #[error("Database Write Failure: {0}")]
    DatabaseWriteError(String),

    // coinbase api errors
    #[error("Coinbase API Error: {0}")]
    CoinbaseAPIError(String),

    #[error("Coinbase Pro API Error: {0}")]
    CoinbaseProAPIError(String),

    // solana errors
    #[error("Solana API Error: {0}")]
    SolanaAPIError(String),

    // sync error
    #[error("Error while syncing client: {0}")]
    SyncError(String),

    // parsing error
    #[error("Could not parse DateTime: {0}")]
    DateTimeParseError(String),
}