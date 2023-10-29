/// All Rain known subgraph endpoints
#[derive(Debug, Clone)]
pub struct Subgraph;

impl Subgraph {
    /// Rain known subgraphs on ethereum mainnet
    pub const ETHEREUM: [&'static str; 2] = [
        "https://api.thegraph.com/subgraphs/name/rainprotocol/interpreter-registry-ethereum",
        "https://api.thegraph.com/subgraphs/name/rainprotocol/interpreter-registry-np-eth"
    ];

    /// Rain known subgraphs on polygon mainnet
    pub const POLYGON: [&'static str; 2] = [
        "https://api.thegraph.com/subgraphs/name/rainprotocol/interpreter-registry-polygon",
        "https://api.thegraph.com/subgraphs/name/rainprotocol/interpreter-registry-np-matic"
    ];

    /// Rain known subgraphs on mumbai mainnet
    pub const MUMBAI: [&'static str; 2] = [
        "https://api.thegraph.com/subgraphs/name/rainprotocol/interpreter-registry",
        "https://api.thegraph.com/subgraphs/name/rainprotocol/interpreter-registry-np"
    ];

    /// Rain NativeParser subgraphs of all implemented networks
    pub const NP: [&'static str; 3] = [
        Self::ETHEREUM[1],
        Self::POLYGON[1],
        Self::MUMBAI[1]
    ];

    /// Rain legacy(non NativeParser) subgraphs of all implemented networks
    pub const LEGACY: [&'static str; 3] = [
        Self::ETHEREUM[0],
        Self::POLYGON[0],
        Self::MUMBAI[0]
    ];

    /// All Rain known subgraph endpoint URLs
    pub const ALL: [&'static str; 6] = [
        Self::ETHEREUM[0],
        Self::ETHEREUM[1],
        Self::POLYGON[0],
        Self::POLYGON[1],
        Self::MUMBAI[0],
        Self::MUMBAI[1]
    ];

    /// get the subgraph endpoint from a chain id
    pub fn of_chain(chain_id: u64) -> anyhow::Result<[&'static str; 2]> {
        match chain_id {
            1 => Ok(Self::ETHEREUM),
            137 => Ok(Self::POLYGON),
            80001 => Ok(Self::MUMBAI),
            _ => Err(anyhow::anyhow!("no rain subgraph is implemented for this network"))
        }
    }
}
