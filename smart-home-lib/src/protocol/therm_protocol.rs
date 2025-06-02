use serde::{Deserialize, Serialize};

/// Данные от термометра по UDP
#[derive(Serialize, Deserialize)]
pub struct ThermData {
    pub temperature: f64,
    pub device_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn therm_data_serialization() {
        // Тест полных данных
        let data = ThermData {
            temperature: 22.5,
            device_id: Some("kitchen_001".to_string()),
        };

        let json = serde_json::to_string(&data).expect("Failed to serialize");
        let expected = r#"{"temperature":22.5,"device_id":"kitchen_001"}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn therm_data_serialization_no_device_id() {
        // Тест данных без device_id
        let data = ThermData {
            temperature: -10.0,
            device_id: None,
        };

        let json = serde_json::to_string(&data).expect("Failed to serialize");
        let expected = r#"{"temperature":-10.0,"device_id":null}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn therm_data_deserialization() {
        // Тест десериализации полных данных
        let json = r#"{"temperature":22.5,"device_id":"kitchen_001"}"#;
        let data: ThermData = serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(data.temperature, 22.5);
        assert_eq!(data.device_id, Some("kitchen_001".to_string()));
    }

    #[test]
    fn therm_data_deserialization_no_device_id() {
        // Тест десериализации без device_id
        let json = r#"{"temperature":-5.5,"device_id":null}"#;
        let data: ThermData = serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(data.temperature, -5.5);
        assert_eq!(data.device_id, None);
    }

    #[test]
    fn therm_data_round_trip() {
        // Тест полного цикла: сериализация -> десериализация
        let original = ThermData {
            temperature: 99.99,
            device_id: Some("test_device_123".to_string()),
        };

        let json = serde_json::to_string(&original).expect("Failed to serialize");
        let restored: ThermData = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(original.temperature, restored.temperature);
        assert_eq!(original.device_id, restored.device_id);
    }

    #[test]
    fn invalid_json_handling() {
        // Тест обработки невалидного JSON
        let invalid_cases = vec![
            r#"{"temperature":"not_a_number","device_id":"test"}"#,
            r#"{"wrong_field":22.5,"device_id":"test"}"#,
            r#"invalid json"#,
            r#"{"temperature":22.5"#, // не закрыт
            r#"{}"#,                  // пустой объект
        ];

        for invalid_json in invalid_cases {
            let result: Result<ThermData, _> = serde_json::from_str(invalid_json);
            assert!(
                result.is_err(),
                "Should fail for invalid JSON: {}",
                invalid_json
            );
        }
    }
}
