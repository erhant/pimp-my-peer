#[derive(Debug, Clone)]
pub enum Keyword {
    StartsWith(String),
    Contains(String),
    EndsWith(String),
}

impl Keyword {
    /// Checks if a given input has the respective keyword.
    pub fn check(&self, input: &str) -> bool {
        match self {
            Self::StartsWith(keyword) => input.starts_with(keyword),
            Self::Contains(keyword) => input.contains(keyword),
            Self::EndsWith(keyword) => input.ends_with(keyword),
        }
    }
}
