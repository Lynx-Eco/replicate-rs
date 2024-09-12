use std::fmt;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("invalid identifier, it must be in the format \"owner/name\" or \"owner/name:version\"")]
pub struct InvalidIdentifierError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier {
    pub owner: String,
    pub name: String,
    pub version: Option<String>,
}

impl Identifier {
    pub fn parse(identifier: &str) -> Result<Self, InvalidIdentifierError> {
        let parts: Vec<&str> = identifier.split('/').collect();
        if parts.len() != 2 {
            return Err(InvalidIdentifierError);
        }

        let owner = parts[0].to_string();
        let (name, version): (String, Option<String>) = if
            let Some((n, v)) = parts[1].split_once(':')
        {
            (n.to_string(), Some(v.to_string()))
        } else {
            (parts[1].to_string(), None)
        };
        if owner.is_empty() || name.is_empty() {
            return Err(InvalidIdentifierError);
        }

        Ok(Identifier {
            owner,
            name,
            version,
        })
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.version {
            Some(v) => write!(f, "{}:{}:{}", self.owner, self.name, v),
            None => write!(f, "{}:{}", self.owner, self.name),
        }
    }
}
