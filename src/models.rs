use schemars;
use serde::{Deserialize, Serialize};

/// Response structure for hot searches
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema, Clone, PartialEq)]
pub struct HotSearchItem {
    #[schemars(description = "Rank position of the hot search")]
    pub rank: usize,
    #[schemars(description = "Title of the hot search")]
    pub title: String,
    #[schemars(description = "Hot score (popularity) of the search")]
    pub score: String,
    #[schemars(description = "URL to the search results")]
    pub url: String,
}

/// Request parameters for fetching hot searches
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct HotSearchRequest {
    #[schemars(description = "Number of hot searches to return (max 50)")]
    #[serde(default = "default_limit")]
    pub limit: usize,
}

/// Default limit for hot search results
pub fn default_limit() -> usize {
    10
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hot_search_item_serialization() {
        let item = HotSearchItem {
            rank: 1,
            title: "Test Topic".to_string(),
            score: "12345".to_string(),
            url: "https://example.com".to_string(),
        };
        
        let serialized = serde_json::to_string(&item).unwrap();
        let deserialized: HotSearchItem = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(item, deserialized);
    }
    
    #[test]
    fn test_hot_search_request_default_limit() {
        let request_json = r#"{}"#;
        let request: HotSearchRequest = serde_json::from_str(request_json).unwrap();
        
        assert_eq!(request.limit, default_limit());
    }
    
    #[test]
    fn test_hot_search_request_custom_limit() {
        let request_json = r#"{"limit": 20}"#;
        let request: HotSearchRequest = serde_json::from_str(request_json).unwrap();
        
        assert_eq!(request.limit, 20);
    }
}