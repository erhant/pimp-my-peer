/// A keyword to be checked.
#[derive(Debug, Clone)]
enum Keyword {
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

#[derive(Debug, Clone)]
pub struct Keywords(Vec<Keyword>);

impl Default for Keywords {
    fn default() -> Self {
        Self::new()
    }
}

impl Keywords {
    pub fn new() -> Self {
        Self(Default::default())
    }

    /// Adds a keyword to be prefix of the peer id.
    #[inline]
    pub fn starts_with(mut self, keyword: &str) -> Self {
        self.0.push(Keyword::StartsWith(keyword.to_string()));
        self
    }

    /// Adds a keyword to be suffix of the peer id.
    #[inline]
    pub fn ends_with(mut self, keyword: &str) -> Self {
        self.0.push(Keyword::EndsWith(keyword.to_string()));
        self
    }

    /// Adds a keyword to be contained within the peer id.
    #[inline]
    pub fn contains(mut self, keyword: &str) -> Self {
        self.0.push(Keyword::Contains(keyword.to_string()));
        self
    }

    /// Checks that the given string contains the keywords.
    #[inline]
    pub fn is_valid(&self, input: &str) -> bool {
        self.0.iter().all(|keyword| keyword.check(input))
    }
}
