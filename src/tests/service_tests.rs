use crate::models::HotSearchItem;
use crate::service::WeiboHotSearch;
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
fn test_integration_weibo_service_creation() {
    init();
    
    // Create a WeiboHotSearch instance
    let service = WeiboHotSearch::new();
    
    // Just verify that the service struct was created
    assert!(!format!("{:?}", service).is_empty());
}

#[test]
fn test_hot_search_item_creation() {
    let item = HotSearchItem {
        rank: 1,
        title: "测试话题".to_string(),
        score: "12345".to_string(),
        url: "https://example.com/test".to_string(),
    };
    
    assert_eq!(item.rank, 1);
    assert_eq!(item.title, "测试话题");
    assert_eq!(item.score, "12345");
    assert_eq!(item.url, "https://example.com/test");
}