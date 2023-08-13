pub struct ToolChain {
    pub name: String,
    pub path: String,
    pub cmd: String,
}

impl ToolChain {
    /// Create a new ToolChain
    pub fn new(name: String, path: String, cmd: String) -> ToolChain {
        ToolChain {
            name,
            path,
            cmd,
        }
    }
}
