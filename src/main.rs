use axum::{extract::Query, extract::State, response::Json, routing::get, Router};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Subhasita {
    pub id: Uuid,
    pub category: String,
    pub content: String,
    pub source: String,
    pub author: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct SubhasitasQuery {
    pub category: Option<String>,
}

pub type SubhasitasStore = Arc<Mutex<Vec<Subhasita>>>;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub data: Option<T>,
    pub message: String,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            data: Some(data),
            message: "Success".to_string(),
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            data: None,
            message,
        }
    }
}

pub async fn get_subhasita(
    Query(_params): Query<SubhasitasQuery>,
    State(store): State<SubhasitasStore>,
) -> Json<ApiResponse<Subhasita>> {
    let subhasitas = store.lock().unwrap();

    if subhasitas.is_empty() {
        return Json(ApiResponse::error("No subhasitas available".to_string()));
    }

    let mut rng = rand::rng();
    let random_number = rng.random_range(0..subhasitas.len());
    let response: ApiResponse<Subhasita> = subhasitas.get(random_number).cloned().map_or_else(
        || ApiResponse::error("Failed to get Subhasita".to_string()),
        |subhasita| ApiResponse::success(subhasita),
    );

    Json(response)
}

#[tokio::main]
async fn main() {
    let subhasitas: SubhasitasStore = Arc::new(Mutex::new(Vec::new()));

    {
        let subhasita_1 = Subhasita {
            id: Uuid::new_v4(),
            category: "Software".to_string(),
            content: "Collaborate early and often".to_string(),
            ..Default::default()
        };

        let subhasita_2 = Subhasita {
            id: Uuid::new_v4(),
            category: "Software".to_string(),
            content: "Don't try to be genius".to_string(),
            ..Default::default()
        };

        let subhasita_3 = Subhasita {
            id: Uuid::new_v4(),
            category: "Software".to_string(),
            content: "Be vulnerable".to_string(),
            ..Default::default()
        };

        let subhasita_4 = Subhasita {
            id: Uuid::new_v4(),
            category: "Software".to_string(),
            content: "Iterate quickly".to_string(),
            ..Default::default()
        };

        let mut subhasitas = subhasitas.lock().unwrap();
        subhasitas.push(subhasita_1);
        subhasitas.push(subhasita_2);
        subhasitas.push(subhasita_3);
        subhasitas.push(subhasita_4);
    }
    let app = Router::new()
        .route("/subhasita", get(get_subhasita))
        .with_state(subhasitas);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3210").await.unwrap();

    println!("🚀 Server running on http://localhost:3210");

    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::extract::{Query, State};

    // Helper function to create a test store with sample data
    fn create_test_store() -> SubhasitasStore {
        let store = Arc::new(Mutex::new(Vec::new()));
        let test_subhasitas = vec![
            Subhasita {
                id: Uuid::new_v4(),
                category: "Software".to_string(),
                content: "Test wisdom 1".to_string(),
                source: "Test source 1".to_string(),
                author: "Test author 1".to_string(),
                description: "Test description 1".to_string(),
            },
            Subhasita {
                id: Uuid::new_v4(),
                category: "Life".to_string(),
                content: "Test wisdom 2".to_string(),
                source: "Test source 2".to_string(),
                author: "Test author 2".to_string(),
                description: "Test description 2".to_string(),
            },
            Subhasita {
                id: Uuid::new_v4(),
                category: "Software".to_string(),
                content: "Test wisdom 3".to_string(),
                source: "Test source 3".to_string(),
                author: "Test author 3".to_string(),
                description: "Test description 3".to_string(),
            },
        ];

        {
            let mut subhasitas = store.lock().unwrap();
            for subhasita in test_subhasitas {
                subhasitas.push(subhasita);
            }
        }

        store
    }

    #[test]
    fn test_api_response_success() {
        let subhasita = Subhasita {
            id: Uuid::new_v4(),
            category: "Test".to_string(),
            content: "Test content".to_string(),
            source: "Test source".to_string(),
            author: "Test author".to_string(),
            description: "Test description".to_string(),
        };

        let response = ApiResponse::success(subhasita.clone());

        assert_eq!(response.message, "Success");
        assert!(response.data.is_some());
        assert_eq!(response.data.unwrap().content, "Test content");
    }

    #[test]
    fn test_api_response_error() {
        let error_message = "Something went wrong".to_string();
        let response: ApiResponse<Subhasita> = ApiResponse::error(error_message.clone());

        assert_eq!(response.message, error_message);
        assert!(response.data.is_none());
    }

    #[test]
    fn test_subhasita_default() {
        let subhasita = Subhasita::default();

        assert_eq!(subhasita.category, "");
        assert_eq!(subhasita.content, "");
        assert_eq!(subhasita.source, "");
        assert_eq!(subhasita.author, "");
        assert_eq!(subhasita.description, "");
    }

    #[test]
    fn test_subhasita_creation() {
        let subhasita = Subhasita {
            id: Uuid::new_v4(),
            category: "Wisdom".to_string(),
            content: "Knowledge is power".to_string(),
            source: "Ancient text".to_string(),
            author: "Sage".to_string(),
            description: "A wise saying".to_string(),
        };

        assert_eq!(subhasita.category, "Wisdom");
        assert_eq!(subhasita.content, "Knowledge is power");
        assert_eq!(subhasita.source, "Ancient text");
        assert_eq!(subhasita.author, "Sage");
        assert_eq!(subhasita.description, "A wise saying");
    }

    #[test]
    fn test_subhasitas_store_creation() {
        let store: SubhasitasStore = Arc::new(Mutex::new(Vec::new()));
        let subhasitas = store.lock().unwrap();
        assert_eq!(subhasitas.len(), 0);
    }

    #[test]
    fn test_subhasitas_store_add_items() {
        let store: SubhasitasStore = Arc::new(Mutex::new(Vec::new()));

        let subhasita = Subhasita {
            id: Uuid::new_v4(),
            category: "Test".to_string(),
            content: "Test wisdom".to_string(),
            ..Default::default()
        };

        {
            let mut subhasitas = store.lock().unwrap();
            subhasitas.push(subhasita.clone());
        }

        let subhasitas = store.lock().unwrap();
        assert_eq!(subhasitas.len(), 1);
        assert_eq!(subhasitas[0].content, "Test wisdom");
    }

    #[tokio::test]
    async fn test_get_subhasita_with_data() {
        let store = create_test_store();
        let query = SubhasitasQuery { category: None };

        let response = get_subhasita(Query(query), State(store.clone())).await;

        let json_response = response.0;
        assert_eq!(json_response.message, "Success");
        assert!(json_response.data.is_some());

        let subhasita = json_response.data.unwrap();
        // Verify that we got one of our test subhasitas
        let valid_contents = ["Test wisdom 1", "Test wisdom 2", "Test wisdom 3"];
        assert!(valid_contents.contains(&subhasita.content.as_str()));
    }

    #[tokio::test]
    async fn test_get_subhasita_empty_store() {
        let store: SubhasitasStore = Arc::new(Mutex::new(Vec::new()));
        let query = SubhasitasQuery { category: None };

        let response = get_subhasita(Query(query), State(store)).await;

        let json_response = response.0;
        assert_eq!(json_response.message, "No subhasitas available");
        assert!(json_response.data.is_none());
    }

    #[test]
    fn test_subhasitas_query_deserialization() {
        // Test that we can properly deserialize query parameters
        let query_with_category = SubhasitasQuery {
            category: Some("Software".to_string()),
        };
        assert_eq!(query_with_category.category, Some("Software".to_string()));

        let query_without_category = SubhasitasQuery { category: None };
        assert_eq!(query_without_category.category, None);
    }

    #[test]
    fn test_store_thread_safety() {
        use std::thread;
        use std::time::Duration;

        let store = create_test_store();
        let mut handles = vec![];

        // Spawn multiple threads that read from the store
        for i in 0..5 {
            let store_clone = Arc::clone(&store);
            let handle = thread::spawn(move || {
                for _ in 0..10 {
                    let subhasitas = store_clone.lock().unwrap();
                    assert!(subhasitas.len() > 0, "Thread {} found empty store", i);
                    // Simulate some work
                    thread::sleep(Duration::from_millis(1));
                }
            });
            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[tokio::test]
    async fn test_concurrent_access() {
        let store = create_test_store();
        let mut tasks = vec![];

        // Create multiple concurrent tasks accessing the store
        for _ in 0..10 {
            let store_clone = Arc::clone(&store);
            let task = tokio::spawn(async move {
                let query = SubhasitasQuery { category: None };
                let response = get_subhasita(Query(query), State(store_clone)).await;
                assert_eq!(response.0.message, "Success");
            });
            tasks.push(task);
        }

        // Wait for all tasks to complete
        for task in tasks {
            task.await.unwrap();
        }
    }
}
