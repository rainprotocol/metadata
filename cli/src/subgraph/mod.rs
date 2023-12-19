/// # Rain Subgraphs
/// all known subgraph endpoints
#[derive(Debug, Clone)]
pub struct KnownSubgraphs;

impl KnownSubgraphs {
    /// Rain known subgraphs on ethereum mainnet
    pub const ETHEREUM: [&'static str; 3] = [
        "https://api.thegraph.com/subgraphs/name/rainlanguage/interpreter-registry-ethereum", // legacy endpoint
        "https://api.thegraph.com/subgraphs/name/rainlanguage/interpreter-registry-np-eth", // np endpoint
        "https://api.thegraph.com/subgraphs/name/rainlanguage/interpreter-registry-npe2-eth", // npe2 endpoint
    ];

    /// Rain known subgraphs on polygon mainnet
    pub const POLYGON: [&'static str; 3] = [
        "https://api.thegraph.com/subgraphs/name/rainlanguage/interpreter-registry-polygon", // legacy endpoint
        "https://api.thegraph.com/subgraphs/name/rainlanguage/interpreter-registry-np-matic", // np endpoint
        "https://api.thegraph.com/subgraphs/name/rainlanguage/interpreter-registry-npe2-mati", // npe2 endpoint
    ];

    /// Rain known subgraphs on mumbai (polygon testnet)
    pub const MUMBAI: [&'static str; 3] = [
        "https://api.thegraph.com/subgraphs/name/rainlanguage/interpreter-registry", // legacy endpoint
        "https://api.thegraph.com/subgraphs/name/rainlanguage/interpreter-registry-np", // np endpoint
        "https://api.thegraph.com/subgraphs/name/rainlanguage/interpreter-registry-npe2", // npe2 endpoint
    ];

    /// Rain NPE2 subgraphs of all suppoerted networks
    pub const NPE2: [&'static str; 3] = [Self::ETHEREUM[2], Self::POLYGON[2], Self::MUMBAI[2]];

    /// Rain NativeParser subgraphs of all suppoerted networks
    pub const NP: [&'static str; 3] = [Self::ETHEREUM[1], Self::POLYGON[1], Self::MUMBAI[1]];

    /// Rain legacy(non NativeParser) subgraphs of all suppoerted networks
    pub const LEGACY: [&'static str; 3] = [Self::ETHEREUM[0], Self::POLYGON[0], Self::MUMBAI[0]];

    /// All Rain known subgraph endpoint URLs
    pub const ALL: [&'static str; 9] = [
        Self::ETHEREUM[0],
        Self::ETHEREUM[1],
        Self::ETHEREUM[2],
        Self::POLYGON[0],
        Self::POLYGON[1],
        Self::POLYGON[2],
        Self::MUMBAI[0],
        Self::MUMBAI[1],
        Self::MUMBAI[2],
    ];

    /// get the subgraph endpoint from a chain id
    pub fn of_chain(chain_id: u64) -> anyhow::Result<[&'static str; 3]> {
        match chain_id {
            1 => Ok(Self::ETHEREUM),
            137 => Ok(Self::POLYGON),
            80001 => Ok(Self::MUMBAI),
            _ => Err(anyhow::anyhow!(
                "no rain subgraph is deployed for this network"
            )),
        }
    }
}
