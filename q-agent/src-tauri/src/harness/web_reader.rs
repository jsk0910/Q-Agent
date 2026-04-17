use reqwest::Client;
use readability::extractor;
use htmd::HtmlToMarkdown;
use std::time::Duration;
use url::Url;

pub struct WebReader;

impl WebReader {
    /// Fetches a URL, extracts the main content using readability, 
    /// and converts it to clean Markdown.
    pub async fn fetch_and_parse(client: &Client, url_str: &str) -> Result<String, String> {
        // 1. Initial Fetch
        let res = client
            .get(url_str)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
            .timeout(Duration::from_secs(15))
            .send()
            .await
            .map_err(|e| format!("Failed to fetch URL: {}", e))?;

        if !res.status().is_success() {
            return Err(format!("Server returned error status: {}", res.status()));
        }

        let html = res.text().await.map_err(|e| format!("Failed to read body: {}", e))?;

        // 2. Specialized GitHub handling
        if url_str.contains("github.com") {
            if let Some(readme) = Self::try_fetch_github_readme(client, url_str).await {
                return Ok(format!("--- GITHUB README CONTENT ---\n\n{}", readme));
            }
        }

        // 3. Extract Main Content (Readability)
        let base_url = Url::parse(url_str).unwrap_or_else(|_| Url::parse("http://localhost").unwrap());
        let mut cursor = std::io::Cursor::new(html.clone());
        
        match extractor::extract(&mut cursor, &base_url) {
            Ok(product) => {
                // 4. Convert to Markdown
                let converter = HtmlToMarkdown::new();
                let markdown = converter.convert(&product.content);
                
                // Add title and metadata for context
                let mut output = format!("# {}\n\n", product.title);
                output.push_str(&markdown);
                
                // Final cleaning to ensure token efficiency
                Ok(Self::post_process_markdown(&output))
            }
            Err(_) => {
                // Fallback: If readability fails, try to convert the whole body but it might be noisy
                let converter = HtmlToMarkdown::new();
                Ok(converter.convert(&html))
            }
        }
    }

    async fn try_fetch_github_readme(client: &Client, url: &str) -> Option<String> {
        let parts: Vec<&str> = url.trim_end_matches('/').split('/').collect();
        if parts.len() < 5 { return None; }
        
        let owner = parts[3];
        let repo = parts[4];

        for branch in ["main", "master"] {
            let raw_url = format!("https://raw.githubusercontent.com/{}/{}/{}/README.md", owner, repo, branch);
            if let Ok(res) = client.get(&raw_url).send().await {
                if res.status().is_success() {
                    if let Ok(text) = res.text().await {
                        return Some(text);
                    }
                }
            }
        }
        None
    }

    fn post_process_markdown(md: &str) -> String {
        // Remove excessive whitespace and redundant empty lines
        let mut result = String::new();
        let mut empty_line_count = 0;

        for line in md.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                empty_line_count += 1;
                if empty_line_count <= 1 {
                    result.push_str("\n");
                }
            } else {
                empty_line_count = 0;
                result.push_str(trimmed);
                result.push_str("\n");
            }
        }
        result.trim().to_string()
    }
}
