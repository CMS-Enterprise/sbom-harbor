/// The Counter struct is used to keep track of
/// what happened to an attempt to submit an SBOM.
///
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Counter {

    /// This value is incremented if the Repo is archived
    pub(crate) archived: i32,

    /// This value is incremented if the Repo is disabled
    pub(crate) disabled: i32,

    /// This value is incremented if the Repo is empty
    pub(crate) empty: i32,

    /// This value is incremented if the Repo is processed successfully
    pub(crate) processed: i32,

    /// This value is incremented if the last commit hash of
    /// the repo is in the database already. This happens when
    /// there has been no change in the repo since last run
    pub(crate) hash_matched: i32,

    /// This value is incremented if there is an error when trying to upload the SBOM.
    pub(crate) upload_errors: i32,
}

/// Default, completely 0'd out default Counter
///
impl Default for Counter {
    fn default() -> Self {
        Self {
            archived: 0,
            disabled: 0,
            empty: 0,
            processed: 0,
            hash_matched: 0,
            upload_errors: 0,
        }
    }
}