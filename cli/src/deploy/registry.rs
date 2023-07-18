use clap::ValueEnum;

#[derive(Debug)]
pub struct Ethereum {
    pub url : String ,
    pub provider : String ,
    pub scan_base_uri : String ,
    pub chain_id : String ,  
    pub block_scanner_api : String, 
    pub block_scanner_key : String
}  

impl Default for Ethereum {
    fn default() -> Ethereum {
        Ethereum { 
            url: String::from("https://api.thegraph.com/subgraphs/name/rainprotocol/interpreter-registry-ethereum") ,
            provider : String::from("https://eth-mainnet.g.alchemy.com/v2/gqp-i6HKrlY8gShHDXkJw-iqudcviIyx") ,
            scan_base_uri : String::from("https://etherscan.io/") ,
            chain_id :  String::from("1") , 
            block_scanner_api : String::from("https://api.etherscan.io/") ,
            block_scanner_key : String::from("2JHMSJCUGUJ86RAKM1EPD15JJ3VAY76464")
        }
    }
}  

#[derive(Debug)]
pub struct Polygon {
    pub url : String ,
    pub provider : String,
    pub scan_base_uri : String ,
    pub chain_id : String ,
    pub block_scanner_api : String, 
    pub block_scanner_key : String
}  

impl Default for Polygon {
    fn default() -> Polygon {
        Polygon { 
            url: String::from("https://api.thegraph.com/subgraphs/name/rainprotocol/interpreter-registry-polygon") ,
            provider : String::from("https://polygon-mainnet.g.alchemy.com/v2/WLWVvo6m4MXAZ3GkzmMI8ZnLIg_bBNaO") ,
            scan_base_uri : String::from("https://polygonscan.com/") ,
            chain_id :  String::from("137") ,
            block_scanner_api : String::from("https://api.polygonscan.com/") ,
            block_scanner_key : String::from("MBFVU16WSKFB9Z5W17HC2DNTY3N6W9SMPX")

        }
    }
}  

#[derive(Debug)]
pub struct Mumbai {
    pub url : String ,
    pub provider : String,
    pub scan_base_uri : String ,
    pub chain_id : String ,
    pub block_scanner_api : String, 
    pub block_scanner_key : String
}  

impl Default for Mumbai {
    fn default() -> Mumbai {
        Mumbai { 
            url: String::from("https://api.thegraph.com/subgraphs/name/rainprotocol/interpreter-registry") ,
            provider : String::from("https://polygon-mumbai.g.alchemy.com/v2/yAwbpk-0UDo-G398kyr6iKrUVWL5fyVj") ,
            scan_base_uri : String::from("https://mumbai.polygonscan.com/") ,
            chain_id :  String::from("80001") ,
            block_scanner_api : String::from("https://api-testnet.polygonscan.com/") ,
            block_scanner_key : String::from("MBFVU16WSKFB9Z5W17HC2DNTY3N6W9SMPX")
        }
    }
}   

#[derive(Debug)]
pub struct Fuji {
    pub provider : String ,
    pub scan_base_uri : String ,
    pub chain_id : String , 
    pub block_scanner_api : String, 
    pub block_scanner_key : String
}  

impl Default for Fuji {
    fn default() -> Fuji {
        Fuji { 
            provider : String::from("https://api.avax-test.network/ext/bc/C/rpc") ,
            scan_base_uri : String::from("https://testnet.snowtrace.io/") ,
            chain_id :  String::from("43113") , 
            block_scanner_api : String::from("https://api-testnet.snowtrace.io/api") ,
            block_scanner_key : String::from("1ANEUH2DZN3C8YDHH9U4UKMYHY2JVFDUQZ")
        }
    }
}  

 #[derive(Debug)]
 #[derive(Clone,ValueEnum)]
pub enum RainNetworks{
    Ethereum,
    Polygon,
    Mumbai,
    Fuji
}  
