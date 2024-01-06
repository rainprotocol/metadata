import { HardhatUserConfig } from "hardhat/config";
import "@typechain/hardhat";
import "@nomiclabs/hardhat-waffle";

function createLocalhostConfig() {
  const url = "http://localhost:8545";
  const mnemonic =
    "test test test test test test test test test test test junk";
  return {
    accounts: {
      count: 10,
      initialIndex: 0,
      mnemonic,
      path: "m/44'/60'/0'/0",
    },
    url,
  };
}

const config: HardhatUserConfig = {
  solidity: "0.8.18",
  typechain: {
    outDir: "typechain", // overrides upstream 'fix' for another issue which changed this to 'typechain-types'
  },
  networks: {
    hardhat: {
      blockGasLimit: 100000000,
      allowUnlimitedContractSize: true,
    },
    mumbai: {
      url: "https://rpc-mumbai.maticvigil.com",
      accounts: process.env["DEPLOYMENT_KEY_MUMBAI"]
        ? [process.env["DEPLOYMENT_KEY_MUMBAI"]]
        : [],
    },
    localhost: createLocalhostConfig()
  },
  defaultNetwork: "localhost",
};

export default config;
