/// Returns a bytes32 string representation of text. If the length of text exceeds 32 bytes,
/// an error is returned.
pub fn format_bytes32_string(text: &str) -> anyhow::Result<[u8; 32]> {
  let str_bytes: &[u8] = text.as_bytes();
  if str_bytes.len() > 32 {
      return Err(anyhow::anyhow!("unexpected length, must be 32 bytes"))
  }

  let mut bytes32: [u8; 32] = [0u8; 32];
  bytes32[..str_bytes.len()].copy_from_slice(str_bytes);

  Ok(bytes32)
}

/// Returns the decoded string represented by the bytes32 encoded data.
pub fn parse_bytes32_string(bytes: &[u8; 32]) -> anyhow::Result<&str> {
  let mut length = 0;
  while length < 32 && bytes[length] != 0 {
      length += 1;
  }
  Ok(std::str::from_utf8(&bytes[..length])?)
}


#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::hex;

    #[test]
    fn bytes32_string_parsing() {
        let text_bytes_list = vec![
            ("", hex!("0000000000000000000000000000000000000000000000000000000000000000")),
            ("A", hex!("4100000000000000000000000000000000000000000000000000000000000000")),
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
            assert_eq!(text, parse_bytes32_string(&bytes).unwrap());
        }
    }

    #[test]
    fn bytes32_string_formatting() {
        let text_bytes_list = vec![
            ("", hex!("0000000000000000000000000000000000000000000000000000000000000000")),
            ("A", hex!("4100000000000000000000000000000000000000000000000000000000000000")),
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
            assert_eq!(bytes, format_bytes32_string(text).unwrap());
        }
    }

    #[test]
    fn bytes32_string_formatting_too_long() {
        assert!(matches!(
            format_bytes32_string("ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456").unwrap_err(),
            anyhow::Error { .. }
        ));
    }

}