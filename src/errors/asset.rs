use thiserror::Error;

#[derive(Error, Debug)]
pub enum AssetManagerError {
    #[error("rpc fetch error {0}")]
    RpcFetchError(String),
    #[error("unknown asset manager error")]
    Unknown,
}

pub type AssetManagerResult<T> = Result<T, AssetManagerError>;
