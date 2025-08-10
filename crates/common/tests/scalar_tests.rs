
use common_core::prelude::scalar::*;
use common_core::prelude::*;

#[cfg(test)]
mod scalar_tests {
    use super::*;

    #[test]
    fn test_compute_pubkey() {
        let scalar = Scalar::from(12345u64);
        let pubkey = scalar.compute_pubkey();
        assert_eq!(pubkey, &scalar * &RISTRETTO_BASEPOINT_POINT);
    }

    #[test]
    fn test_ristretto_point_byte_conversion_roundtrip() {
        let scalar = Scalar::from(67890u64);
        let point = scalar.compute_pubkey();
        
        let bytes = point.to_bytes();
        let recovered_point = RistrettoPoint::from_bytes(&bytes).unwrap();
        
        assert_eq!(point, recovered_point);
    }

    #[test]
    fn test_ristretto_point_base58_conversion_roundtrip() {
        let scalar = Scalar::from(112233u64);
        let point = scalar.compute_pubkey();

        let base58_str = point.to_base58();
        let recovered_point = RistrettoPoint::from_base58(base58_str).unwrap();

        assert_eq!(point, recovered_point);
    }

    #[test]
    fn test_from_bytes_invalid_length() {
        let bytes = vec![0u8; 31];
        let result = RistrettoPoint::from_bytes(&bytes);
        assert!(result.is_err());
    }

    #[test]
    fn test_from_bytes_invalid_point() {
        // This is a valid 32-byte array, but it doesn't represent a valid point on the curve.
        // (Specifically, it corresponds to a point with a negative x-coordinate, which is not canonical).
        let bytes = [
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0x10,
        ];
        let result = RistrettoPoint::from_bytes(&bytes);
        assert!(result.is_err());
    }
}
