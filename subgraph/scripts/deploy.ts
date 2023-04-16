import { execSync } from "child_process";
import * as dotenv from "dotenv";
import * as path from "path";
import * as fs from "fs";
import { program } from "commander";
import { ethers, network } from "hardhat";

dotenv.config();

interface DeployConfig {
  configPath: string;
  subgraphName: string;
  versionLabel: string;
  endpoint: string;
  ipfsEndpoint: string;
}

/**
 * Execute Child Processes
 * @param cmd Command to execute
 * @returns The command ran it
 */
const exec = (cmd: string): string | Buffer => {
  const srcDir = path.join(__dirname, "..");
  try {
    return execSync(cmd, { cwd: srcDir, stdio: "inherit" });
  } catch (e) {
    throw new Error(`Failed to run command \`${cmd}\``);
  }
};


const writeFile = (_path: string, file: any): void => {
  try {
    fs.writeFileSync(_path, file);
  } catch (error) {
    console.log(error);
  }
};

const getRPCURL = (network: string): string => {
  if(network == "mumbai") return "https://rpc-mumbai.maticvigil.com";
  if(network == "mainnet") return "https://eth.llamarpc.com";
  if(network == "goerli") return "https://rpc.ankr.com/eth_goerli";
  if(network == "polygon") return "https://polygon-rpc.com";
  return "http://localhost:8545"
}

const checkContract = async (address: string, network: string) => {
  const provider = new ethers.providers.JsonRpcProvider(getRPCURL(network));

  const code = await provider.getCode(address);
  if(code.length < 3) throw new Error("Contract address does not have any bytecode.");
}


const main = async () => {
  program
    .requiredOption("--contractAddress <string>", "Smart contract address")
    .requiredOption(
      "--network <string>",
      "Block Number to start indexing from.",
    )
    .requiredOption(
      "--blockNumber <string>",
      "Block Number to start indexing from.",
    )
    .requiredOption(
      "--subgraphName <string>",
      "The subgraph name to deploy. Eg: 'user/name'."
    )
    .requiredOption(
      "--graphAccessToken <string>",
      "Graph access token for graph auth"
    )
    .option(
      "--subgraphTemplate <string>",
      "Specify a path to yaml file to be used as template. By the default use the root template.",
      "subgraph.template.yaml"
    );

  program.parse();

  
  const options = program.opts();

  await checkContract(options.contractAddress, options.network);

  const _network = options.network;
  const _contractAddress = options.contractAddress;
  const _blockNumber = options.blockNumber;
  const _graphAccessToken = options.graphAccessToken;
  const _subgraphName = options.subgraphName;
  const _endpoint = "--node https://api.thegraph.com/deploy/";
  const _subgraphTemplate = options.subgraphTemplate;

  // Add the address to the subgraph.yaml file

  exec(`npx graph auth --product hosted-service ${_graphAccessToken}`)

  let config = { MetaBoard: _contractAddress, MetaBoardBlock: _blockNumber, network: _network };

  writeFile(path.resolve(__dirname, `../config/${_network}.json`) ,JSON.stringify(config, null, 2))

  exec(
    `npx mustache config/${_network}.json ${_subgraphTemplate} subgraph.yaml`
  );

  // Generate all teh SG code
  exec("npx graph codegen && npx graph build");

  // Deploy the Subgraph
  exec(
    `npx graph deploy ${_endpoint} ${_subgraphName}`
  );
};

main()
  .then(() => {
    const exit = process.exit;
    exit(0);
  })
  .catch((error) => {
    console.error(error);
    const exit = process.exit;
    exit(1);
  });
