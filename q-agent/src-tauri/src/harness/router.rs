pub struct KeywordGuidelineRouter;

impl KeywordGuidelineRouter {
    /// Selects relevant guidelines from a pre-defined set based on simple keyword overlap.
    pub fn route(prompt: &str, available_guidelines: &[(String, String, Vec<String>)]) -> String {
        let prompt_lower = prompt.to_lowercase();
        let mut selected = Vec::new();

        for (_id, content, keywords) in available_guidelines {
            for kw in keywords {
                if prompt_lower.contains(&kw.to_lowercase()) {
                    selected.push(content.clone());
                    break;
                }
            }
        }

        // If none match, allow fallback empty string (or default could be added later)
        if selected.is_empty() {
            return String::new();
        }

        selected.join("\n\n")
    }
}
