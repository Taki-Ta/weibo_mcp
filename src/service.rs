use crate::models::{HotSearchItem, HotSearchRequest};
use reqwest::Client;
use rmcp::{model::ServerInfo, tool, tool_box, ServerHandler};
use scraper::{Html, Selector};
use std::time::Duration;

/// Service for fetching Weibo hot searches
#[derive(Debug, Clone)]
pub struct WeiboHotSearch {
    pub client: Client,
}

impl WeiboHotSearch {
    /// Create a new instance of the service
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
                .timeout(Duration::from_secs(10))
                .build()
                .unwrap(),
        }
    }

    /// Fetches hot searches from Weibo
    pub async fn fetch_hot_searches(&self, limit: usize) -> Result<Vec<HotSearchItem>, String> {
        // Cap the limit to 50
        let limit = limit.min(50);
        
        // Fetch the Weibo hot search page
        let response = self.client
            .get("https://s.weibo.com/top/summary")
            .send()
            .await
            .map_err(|e| format!("Failed to request Weibo: {}", e))?;
        
        let html = response
            .text()
            .await
            .map_err(|e| format!("Failed to get response text: {}", e))?;
        
        // Parse the HTML
        let document = Html::parse_document(&html);
        let row_selector = Selector::parse("#pl_top_realtimehot table tbody tr")
            .map_err(|e| format!("Failed to parse selector: {}", e))?;
        
        let title_selector = Selector::parse("td.td-02 a")
            .map_err(|e| format!("Failed to parse title selector: {}", e))?;
        
        let score_selector = Selector::parse("td.td-02 span")
            .map_err(|e| format!("Failed to parse score selector: {}", e))?;
        
        // Extract hot searches
        let mut hot_searches = Vec::new();
        for (idx, row) in document.select(&row_selector).enumerate() {
            if idx == 0 || hot_searches.len() >= limit {
                // Skip header row
                continue;
            }
            
            if let Some(title_element) = row.select(&title_selector).next() {
                let title = title_element.text().collect::<Vec<_>>().join("").trim().to_string();
                let url = format!(
                    "https://s.weibo.com/weibo?q={}", 
                    urlencoding::encode(&title)
                );
                
                let score = row.select(&score_selector)
                    .next()
                    .map_or("N/A".to_string(), |e| e.text().collect::<Vec<_>>().join("").trim().to_string());
                
                hot_searches.push(HotSearchItem {
                    rank: idx,
                    title,
                    score,
                    url,
                });
            }
            
            if hot_searches.len() >= limit {
                break;
            }
        }
        
        Ok(hot_searches)
    }

    /// Tool to fetch hot searches - exposed through MCP
    #[tool(description = "Fetch current Weibo hot searches")]
    async fn get_hot_searches(
        &self,
        #[tool(aggr)]
        HotSearchRequest { limit }: HotSearchRequest,
    ) -> String {
        match self.fetch_hot_searches(limit).await {
            Ok(hot_searches) => match serde_json::to_string_pretty(&hot_searches) {
                Ok(json) => json,
                Err(e) => format!("Failed to serialize hot searches: {}", e),
            },
            Err(e) => e,
        }
    }

    tool_box!(WeiboHotSearch { get_hot_searches });
}

impl ServerHandler for WeiboHotSearch {
    tool_box!(@derive);
    
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("Get the current trending topics on Weibo (Chinese social media platform)".into()),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Once;
    
    static INIT: Once = Once::new();
    
    // Initialize the testing environment
    fn init() {
        INIT.call_once(|| {
            // Initialize tracing for tests if needed
            let _ = tracing_subscriber::fmt().try_init();
        });
    }
    
    #[test]
    fn test_parse_hot_searches() {
        init();
        
        // Create test HTML content
        let html = get_mock_html();
        
        // Parse document manually (without HTTP requests)
        let document = Html::parse_document(&html);
        
        // Use the same logic as in fetch_hot_searches but without the async parts
        let row_selector = Selector::parse("#pl_top_realtimehot table tbody tr").unwrap();
        let title_selector = Selector::parse("td.td-02 a").unwrap();
        let score_selector = Selector::parse("td.td-02 span").unwrap();
        
        let mut hot_searches = Vec::new();
        for (idx, row) in document.select(&row_selector).enumerate() {
            if idx == 0 {
                // Skip header row
                continue;
            }
            
            if let Some(title_element) = row.select(&title_selector).next() {
                let title = title_element.text().collect::<Vec<_>>().join("").trim().to_string();
                let url = format!(
                    "https://s.weibo.com/weibo?q={}", 
                    urlencoding::encode(&title)
                );
                
                let score = row.select(&score_selector)
                    .next()
                    .map_or("N/A".to_string(), |e| e.text().collect::<Vec<_>>().join("").trim().to_string());
                
                hot_searches.push(HotSearchItem {
                    rank: idx,
                    title,
                    score,
                    url,
                });
            }
            
            if hot_searches.len() >= 3 {
                break;
            }
        }
        
        // Validate parsing results
        assert_eq!(hot_searches.len(), 3);
        assert_eq!(hot_searches[0].title, "测试话题1");
        assert_eq!(hot_searches[0].score, "50000");
        assert_eq!(hot_searches[1].title, "测试话题2");
        assert_eq!(hot_searches[1].score, "40000");
        assert_eq!(hot_searches[2].title, "测试话题3");
        assert_eq!(hot_searches[2].score, "30000");
    }
    
    fn get_mock_html() -> String {
        r#"
        <html>
        <body>
            <div id="pl_top_realtimehot">
                <table>
                    <tbody>
                        <tr class="title">
                            <td class="td-01">排名</td>
                            <td class="td-02">标题</td>
                            <td class="td-03">热度</td>
                        </tr>
                        <tr>
                            <td class="td-01">1</td>
                            <td class="td-02">
                                <a href="/weibo?q=%23测试话题1%23">测试话题1</a>
                                <span>50000</span>
                            </td>
                        </tr>
                        <tr>
                            <td class="td-01">2</td>
                            <td class="td-02">
                                <a href="/weibo?q=%23测试话题2%23">测试话题2</a>
                                <span>40000</span>
                            </td>
                        </tr>
                        <tr>
                            <td class="td-01">3</td>
                            <td class="td-02">
                                <a href="/weibo?q=%23测试话题3%23">测试话题3</a>
                                <span>30000</span>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </body>
        </html>
        "#.to_string()
    }
}