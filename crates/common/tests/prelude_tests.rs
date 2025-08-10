use common_core::prelude::*;
use serde::{Serialize, Deserialize};

#[cfg(test)]
mod prelude_tests {
    use super::*;

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct TestStruct {
        #[serde(with = "base58")]
        data: Vec<u8>,
    }

    #[test]
    fn test_base58_serialization_deserialization() {
        let original = TestStruct {
            data: vec![1, 2, 3, 4, 5],
        };

        let serialized = serde_json::to_string(&original).unwrap();
        let expected_json = r#"{"data":"7bWpTW"}"#;
        assert_eq!(serialized, expected_json);

        let deserialized: TestStruct = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_data_to_qr_png_creates_valid_base64_png() {
        let data = b"Hello, world!";
        let result = data_to_qr_png(data).unwrap();

        assert!(result.starts_with("data:image/png;base64,"));
        
        let base64_part = result.strip_prefix("data:image/png;base64,").unwrap();
        let png_data = BS64ENGINE.decode(base64_part).unwrap();

        // Check for PNG header
        assert_eq!(&png_data[0..8], &[137, 80, 78, 71, 13, 10, 26, 10]);
    }

    #[test]
    fn test_anyhow_msg_wrapper() {
        let error = msg("This is a test error");
        assert_eq!(error.to_string(), "This is a test error");
    }
}
