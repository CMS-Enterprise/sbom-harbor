
use regex::Regex;
use std::fmt;
use crate::Error;

/// This Enum specifies the types of names that are supported
pub enum NameKind {
    /// Select this value when a safe S3 key name is needed
    S3KeyName,
    /// Select this value when a safe filename name is needed
    FileName,
    /// Select this value when a safe HTTP header name is needed
    HeaderName,
}

impl fmt::Display for NameKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NameKind::S3KeyName => write!(f, "S3KeyName"),
            NameKind::FileName => write!(f, "FileName"),
            NameKind::HeaderName => write!(f, "HeaderName")
        }
    }
}

/// A tiny struct to put all of the naming safety code in
/// so that it is easy to write an object with said name
/// to whatever storage medium is specified.
pub struct NameHelper {
    name: String
}

impl NameHelper {

    /// This method sets the name string to state for the
    /// make_a_safe() method to operate on.
    pub fn from(name: &str) -> Self {
        Self {
            name: String::from(name)
        }
    }

    /// This method creates a safe version of a string for the Kind of Storage
    /// medium that is passed to it.
    pub fn make_a_safe(&self, name_kind: NameKind) -> Result<String, Error> {
        match name_kind {
            NameKind::S3KeyName => self.make_a_safe_s3_key(),
            NameKind::FileName => self.make_a_safe_filename(),
            NameKind::HeaderName => self.make_a_safe_header()
        }
    }

    fn make_a_safe_s3_key(&self) -> Result<String, Error> {
        self.simple_search_and_replace()
    }

    fn make_a_safe_filename(&self) -> Result<String, Error> {
        self.simple_search_and_replace()
    }

    fn make_a_safe_header(&self) -> Result<String, Error> {
        self.simple_search_and_replace()
    }

    /// The only strategy so far.  This may be all we need for now,
    /// but we can add different strategies as we realize what names
    /// must be on these various mediums.
    ///
    /// Note: The return type is a Result for future use.  The current
    /// implementation does not need it, but it's there now to prevent
    /// signature changes in the future.
    fn simple_search_and_replace(&self) -> Result<String, Error> {
        let re = Regex::new(r"[^A-Za-z0-9]").unwrap();
        Ok(re.replace_all(self.name.as_str(), "-").to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::Error;
    use crate::naming::NameHelper;
    use crate::naming::NameKind::{FileName, HeaderName, S3KeyName};

    #[async_std::test]
    async fn can_format_header_name() -> Result<(), Error> {
        let invalid = "some::invalid";

        let valid = NameHelper::from(invalid).make_a_safe(HeaderName)?;

        assert!(!valid.contains(':'));
        assert!(valid.contains('-'));

        Ok(())
    }

    #[async_std::test]
    async fn can_format_file_name() -> Result<(), Error> {
        let invalid = "some::invalid";

        let valid = NameHelper::from(invalid).make_a_safe(FileName)?;

        assert!(!valid.contains(':'));
        assert!(valid.contains('-'));

        Ok(())
    }

    #[async_std::test]
    async fn can_format_s3_key_name() -> Result<(), Error> {
        let invalid = "some::invalid";

        let valid = NameHelper::from(invalid).make_a_safe(S3KeyName)?;

        assert!(!valid.contains(':'));
        assert!(valid.contains('-'));

        Ok(())
    }
}