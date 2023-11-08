/// converts string to bytes32
pub fn str_to_bytes32(text: &str) -> anyhow::Result<[u8; 32]> {
    let bytes: &[u8] = text.as_bytes();
    if bytes.len() > 32 {
        return Err(anyhow::anyhow!(
            "unexpected length, must be 32 bytes or less"
        ));
    }
    let mut b32 = [0u8; 32];
    b32[..bytes.len()].copy_from_slice(bytes);
    Ok(b32)
}

/// converts bytes32 to string
pub fn bytes32_to_str(bytes: &[u8; 32]) -> anyhow::Result<&str> {
    let mut len = 32;
    if let Some((pos, _)) = itertools::Itertools::find_position(&mut bytes.iter(), |b| **b == 0u8) {
        len = pos;
    };
    Ok(std::str::from_utf8(&bytes[..len])?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::hex;

    #[test]
    fn test_bytes32_to_str() {
        let text_bytes_list = vec![
            (
                "",
                hex!("0000000000000000000000000000000000000000000000000000000000000000"),
            ),
            (
                "A",
                hex!("4100000000000000000000000000000000000000000000000000000000000000"),
            ),
            (
                "ABCDEFGHIJKLMNOPQRSTUVWXYZ012345",
                hex!("4142434445464748494a4b4c4d4e4f505152535455565758595a303132333435"),
            ),
            (
                "!@#$%^&*(),./;'[]",
                hex!("21402324255e262a28292c2e2f3b275b5d000000000000000000000000000000"),
            ),
        ];

        for (text, bytes) in text_bytes_list {
            assert_eq!(text, bytes32_to_str(&bytes).unwrap());
        }
    }

    #[test]
    fn test_str_to_bytes32() {
        let text_bytes_list = vec![
            (
                "",
                hex!("0000000000000000000000000000000000000000000000000000000000000000"),
            ),
            (
                "A",
                hex!("4100000000000000000000000000000000000000000000000000000000000000"),
            ),
            (
                "ABCDEFGHIJKLMNOPQRSTUVWXYZ012345",
                hex!("4142434445464748494a4b4c4d4e4f505152535455565758595a303132333435"),
            ),
            (
                "!@#$%^&*(),./;'[]",
                hex!("21402324255e262a28292c2e2f3b275b5d000000000000000000000000000000"),
            ),
        ];

        for (text, bytes) in text_bytes_list {
            assert_eq!(bytes, str_to_bytes32(text).unwrap());
        }
    }

    #[test]
    fn test_str_to_bytes32_long() {
        assert!(matches!(
            str_to_bytes32("ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456").unwrap_err(),
            anyhow::Error { .. }
        ));
    }
}
