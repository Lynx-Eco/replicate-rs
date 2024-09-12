use serde::{ Deserialize, Serialize };

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "starting")]
    Starting,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "succeeded")]
    Succeeded,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "canceled")]
    Canceled,
}

impl Status {
    pub fn is_terminated(&self) -> bool {
        matches!(self, Status::Succeeded | Status::Failed | Status::Canceled)
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Starting => write!(f, "starting"),
            Status::Processing => write!(f, "processing"),
            Status::Succeeded => write!(f, "succeeded"),
            Status::Failed => write!(f, "failed"),
            Status::Canceled => write!(f, "canceled"),
        }
    }
}
