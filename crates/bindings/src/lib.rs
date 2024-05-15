use alloy_sol_types::sol;

sol!(
    #![sol(all_derives = true)]
    IDescribedByMetaV1,
    "../../out/IDescribedByMetaV1.sol/IDescribedByMetaV1.json"
);

sol! {
    /// IERC165 supportsInterface fn
    function supportsInterface(bytes4 interfaceId) public view virtual override returns (bool);
}

/// get interface id of IDescribedByMetaV1
pub fn i_described_by_meta_v1_interface_id() -> [u8; 4] {
    let selectors = IDescribedByMetaV1::IDescribedByMetaV1Calls::SELECTORS;
    let mut result = u32::from_be_bytes(selectors[0]);
    for selector in &selectors[1..] {
        result ^= u32::from_be_bytes(*selector);
    }
    result.to_be_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interface_id() {
        // known IDescribeByMetaV1 interface id
        let expected: [u8; 4] = 0x6f5aa28du32.to_be_bytes();
        let result = i_described_by_meta_v1_interface_id();
        assert_eq!(result, expected);
    }
}
