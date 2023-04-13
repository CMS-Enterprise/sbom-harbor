/// A Project is a named entity that can contain 1 child type:
/// - [Codebase]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Project {
    /// The unique identifier for the Project.
    pub id: String,
    /// The name of the project.
    pub name: String,
    /// The FISMA ID for the project.
    pub fisma: String,
    /// Codebases owned by the project.
    #[serde(default = "Vec::new")]
    pub codebases: Vec<Codebase>,
}

impl Project {
    /// Constructor function for creating new [Project] instances.
    pub fn new(name: String, fisma: Option<String>) -> Self {
        Self {
            id: "".to_string(),
            name,
            fisma: fisma.unwrap_or("".to_string()),
            codebases: vec![],
        }
    }

    /// Add a [Codebase] to the codebases Vector.
    pub fn codebases(&mut self, codebase: Codebase) -> &Self {
        self.codebases.push(codebase);
        self
    }
}
