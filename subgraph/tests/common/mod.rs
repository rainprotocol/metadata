pub mod cbor;

use ethers::prelude::k256::ecdsa::SigningKey;
use ethers::prelude::{SignerMiddleware, ContractFactory};
use ethers::signers::{LocalWallet, Signer, Wallet};
use ethers::types::{BlockNumber, U256};
use ethers::{ contract::Contract , providers::Provider};
use ethers_providers::{Http, Middleware};
use ethers_solc::{ConfigurableArtifacts, Artifact};
use ethers_solc::Project;
use ethers_solc::ProjectCompileOutput;
use ethers_solc::ProjectPathsConfig;
use eyre::eyre;
use eyre::Result;
use std::path::PathBuf;
use eyre::ContextCompat;

pub fn get_rpc_provider(rpc_url: &str) -> Provider<Http> {
    let provider =
        Provider::try_from(rpc_url).expect(format!("no prc node found at {rpc_url}").as_str());
    provider
}

pub async fn compile(root: &str) -> Result<ProjectCompileOutput<ConfigurableArtifacts>> {
    // Create path from string and check if the path exists
    let root = PathBuf::from(root);
    if !root.exists() {
        return Err(eyre!("Project root {root:?} does not exists!"));
    }

    // Configure `root` as our project root
    let paths = ProjectPathsConfig::builder()
        .root(&root)
        .sources(&root)
        .build()?;

    // Create a solc ProjectBuilder instance for compilation
    let project = Project::builder()
        .paths(paths)
        .set_auto_detect(true) // auto detect solc version from solidity source code
        .no_artifacts()
        .build()?;

    // Compile project
    let output = project.compile()?;

    // Check for compilation errors
    if output.has_compiler_errors() {
        Err(eyre!(
            "Compiling solidity project failed: {:?}",
            output.output().errors
        ))
    } else {
        Ok(output.clone())
    }
}

pub async fn deploy_meta_board(project: ProjectCompileOutput, deployer_wallet: Wallet<SigningKey>) -> Result<ethers::contract::Contract<SignerMiddleware<ethers_providers::Provider<Http>, Wallet<SigningKey>>>, eyre::ErrReport> {

    let meta_board = project.find("MetaBoard").context("Contract MetaBoard not found")?.clone();

    let provider = get_rpc_provider("http://localhost:8545");

    let chain_id = provider.get_chainid().await?.as_u64();

    let (abi, bytecode, _) = meta_board.into_parts();
    let abi = abi.context("Missing ABI from contract")?;
    let bytecode = bytecode.context("Missing bytecode from contract")?;

    let wallet = deployer_wallet.with_chain_id(chain_id);

    let client = SignerMiddleware::new(provider.clone(), wallet).into();

    let factory = ContractFactory::new(abi.clone(), bytecode, client);

    let mut deployer = factory.deploy(())?;

    let block = provider.clone()
    .get_block(BlockNumber::Latest).await?
    .context("Failed to get latest Block")?;

    let gas_price = block
    .next_block_base_fee()
    .context("Failed to get the next block base fee")?;

    deployer.tx.set_gas_price::<U256>(gas_price);

    let contract: Contract<SignerMiddleware<Provider<Http>, _>> = deployer.clone().legacy().send().await?;

    Ok(contract)
}

pub fn get_signers() -> Result<Vec<Wallet<SigningKey>>> {
    let private_keys = [
        "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80",
        "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d",
        "0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a",
        "0x7c852118294e51e653712a81e05800f419141751be58f605c371e15141b007a6",
        "0x47e179ec197488593b187f80a00eb0da91f1b9d0b13f8733639f19c30a34926a",
        "0x8b3a350cf5c34c9194ca85829a2df0ec3153be0318b5e2d3348e872092edffba",
        "0x92db14e403b83dfe3df233f83dfa3a0d7096f21ca9b0d6d6b8d88b2b4ec1564e",
        "0x4bbbf85ce3377467afe5d46f804f221813b2bb87f24d81f60f1fcdbf7cbf4356",
        "0xdbda1821b80551c9d65939329250298aa3472ba22feea921c0cf5d620ea67b97",
        "0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6",
        "0xf214f2b2cd398c806f84e317254e0f0b801d0643303237d97a22a48e01628897",
        "0x701b615bbdfb9de65240bc28bd21bbc0d996645a3dd57e7b12bc2bdf6f192c82",
        "0xa267530f49f8280200edf313ee7af6b827f2a8bce2897751d06a843f644967b1",
        "0x47c99abed3324a2707c28affff1267e45918ec8c3f20b8aa892e8b065d2942dd",
        "0xc526ee95bf44d8fc405a158bb884d9d1238d99f0612e9f33d006bb0789009aaa",
        "0x8166f546bab6da521a8369cab06c5d2b9e46670292d85c875ee9ec20e84ffb61",
        "0xea6c44ac03bff858b476bba40716402b03e41b8e97e276d1baec7c37d42484a0",
        "0x689af8efa8c651a91ad287602527f3af2fe9f6501a7ac4b061667b5a93e037fd",
        "0xde9be858da4a475276426320d5e9262ecfc3ba460bfac56360bfa6c4c28b4ee0",
        "0xdf57089febbacf7ba0bc227dafbffa9fc08a93fdc68e1e42411a14efcf23656e"
    ];

    let mut signers: Vec<Wallet<SigningKey>> = vec![];

    for key in private_keys.iter() {
        let wallet = key.replace("0x", "").as_str().parse::<LocalWallet>()?;
        signers.push(wallet);
    }

    Ok(signers)
}
