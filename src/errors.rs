use serde::Deserialize;
#[derive(Debug, thiserror::Error, Clone, Deserialize)]
pub enum ApiError {
    #[error("{0}")]
    AppError(String),
}
impl ApiError {
    pub fn to_vec_string(&self) -> Vec<String> {
        match self {
            ApiError::AppError(json) => {
                vec![format!("{}", json)]
            }
        }
    }
}
