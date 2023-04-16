import {
  appendRainMetaDoc,
  exec,
  fetchFile,
  fetchSubgraph,
  getEventArgs,
  MAGIC_NUMBERS,
  META_MAGIC_NUMBER_V1,
  waitForGraphNode,
  waitForSubgraphToBeSynced,
  writeFile,
} from "./utils";
import { MetaBoard, MetaV1Event } from "../typechain/MetaBoard";
import { ApolloFetch, FetchResult } from "apollo-fetch";
import assert from "assert";
import { artifacts, ethers } from "hardhat";
import * as path from "path";
import { cborEncode } from "./cbor";

const { arrayify, toUtf8Bytes } = ethers.utils;

describe("MetaBoard MetaV1 event tests", () => {
  let metaCount = 0;
  let metaBoard: MetaBoard;
  let subgraph: ApolloFetch;
  before(async () => {
    await waitForGraphNode();
    let MetaBoard = await ethers.getContractFactory("MetaBoard");
    metaBoard = (await MetaBoard.deploy()) as MetaBoard;
    await metaBoard.deployed();
    let configPath = path.resolve(__dirname, "../config/localhost.json");
    const config = JSON.parse(fetchFile(configPath));
    config.network = "localhost";
    config.MetaBoard = metaBoard.address;
    config.MetaBoardBlock = metaBoard.deployTransaction.blockNumber;
    writeFile(configPath, JSON.stringify(config, null, 2));
    // create subgraph instance
    exec("graph create --node http://localhost:8020/ test/test");
    // prepare subgraph manifest
    exec(
      "npx mustache config/localhost.json subgraph.template.yaml subgraph.yaml"
    );
    // deploy subgraph
    exec(
      "graph deploy --node http://localhost:8020/ --ipfs http://localhost:5001 test/test  --version-label 1"
    );
    subgraph = fetchSubgraph("test/test");
  });

  it("Should emit emitMeta event", async () => {
    // This get the ABI (which is an object) and stringify it to a JSON string.
    const abiString = JSON.stringify(
      (await artifacts.readArtifact("MetaBoard")).abi,
      null,
      2
    );

    // Take an string and convert it to an Uint8Array representation (UTF-8 bytes)
    const abiU8A = toUtf8Bytes(abiString);

    // Using the Uint8Array representation (which is a byte representation), BUT
    // using the buffer, so the CBOR encoder on Javascript could encoding it as
    // bytes
    const abiBytes = arrayify(abiU8A).buffer;

    const encodedData = cborEncode(
      abiBytes,
      MAGIC_NUMBERS.SOLIDITY_ABIV2,
      "application/json"
    );

    let trx = await metaBoard.emitMeta(1, appendRainMetaDoc(encodedData));
    metaCount++;
    const { sender, meta } = (await getEventArgs(
      trx,
      "MetaV1",
      metaBoard
    )) as MetaV1Event["args"];

    await waitForSubgraphToBeSynced();

    const query = `{
      metaBoard(id: "${metaBoard.address.toLowerCase()}"){
        id
        metaCount
        metas{
          id
        }
      }
    }`;

    const response = (await subgraph({ query })) as FetchResult;
    const metaBoardData = response.data.metaBoard;

    assert.equal(metaBoard.address.toLowerCase(), metaBoardData.id);
    assert.equal(metaBoardData.metas.length, 1, "Wrong meta array length");
    assert.equal(metaBoardData.metaCount, 1, "Wrong meta array length");

    const metaQuery = `{
        metaV1(id: "${trx.hash.toLowerCase()}"){
            id
            meta
            sender
            metaBoard {
              id
            }
            payload
            contentType
            magicNumber
            subject
        }
      }`;

    const metaResponse = (await subgraph({ query: metaQuery })) as FetchResult;
    const metaData = metaResponse.data.metaV1;

    assert.equal(metaData.id, trx.hash, "wrong meta id");
    assert.equal(metaData.meta, meta);
    assert.equal(metaData.sender, sender.toLowerCase());
    assert.equal(metaData.metaBoard.id, metaBoard.address.toLowerCase());
    // assert.equal(metaData.payload, abiBytes);
    assert.equal(metaData.contentType, "application/json");
    assert.equal(metaData.magicNumber, MAGIC_NUMBERS.SOLIDITY_ABIV2);
    assert.equal(metaData.subject, 1, "Wrong subject");
  });

  it("Should emit emitMeta event with diff signers", async () => {
    const abiString = JSON.stringify(
      (await artifacts.readArtifact("MetaBoard")).abi,
      null,
      2
    );

    const abiU8A = toUtf8Bytes(abiString);

    const abiBytes = arrayify(abiU8A).buffer;

    const encodedData = cborEncode(
      abiBytes,
      MAGIC_NUMBERS.SOLIDITY_ABIV2,
      "application/json"
    );

    const signers = await ethers.getSigners();

    const eventEmitter = signers[2];

    const encodedMeta = appendRainMetaDoc(encodedData);

    let trx = await metaBoard.connect(eventEmitter).emitMeta(2, encodedMeta);
    metaCount++;

    await waitForSubgraphToBeSynced();

    const query = `{
      metaBoard(id: "${metaBoard.address.toLowerCase()}"){
        id
        metaCount
        metas{
          id
        }
      }
    }`;

    const response = (await subgraph({ query })) as FetchResult;
    const metaBoardData = response.data.metaBoard;

    assert.equal(metaBoard.address.toLowerCase(), metaBoardData.id);
    assert.equal(metaBoardData.metas.length, 2, "Wrong meta array length");
    assert.equal(metaBoardData.metaCount, 2, "Wrong meta array length");

    const metaQuery = `{
        metaV1(id: "${trx.hash.toLowerCase()}"){
            id
            meta
            sender
            metaBoard {
              id
            }
            payload
            contentType
            magicNumber
            subject
        }
      }`;

    const metaResponse = (await subgraph({ query: metaQuery })) as FetchResult;
    const metaData = metaResponse.data.metaV1;

    assert.equal(metaData.id, trx.hash, "wrong meta id");
    assert.equal(metaData.meta, encodedMeta);
    assert.equal(metaData.sender, eventEmitter.address.toLowerCase());
    assert.equal(metaData.metaBoard.id, metaBoard.address.toLowerCase());    
    // assert.equal(metaData.payload, abiBytes);
    assert.equal(metaData.contentType, "application/json");
    assert.equal(metaData.magicNumber, MAGIC_NUMBERS.SOLIDITY_ABIV2);
    assert.equal(metaData.subject, 2, "Wrong subject");

  });

  it("Should add 10 new notices", async () => {
    for (let i = 0; i < 10; i++) {
      const encodedData = cborEncode(
        "Hello",
        MAGIC_NUMBERS.SOLIDITY_ABIV2,
        "text",
      );

      await metaBoard.emitMeta(i, appendRainMetaDoc(encodedData));
      metaCount++;
    }

    await waitForSubgraphToBeSynced();

    const query = `{
        metaBoard(id: "${metaBoard.address.toLowerCase()}"){
          metaCount
        }
      }`;

    const response = (await subgraph({ query })) as FetchResult;
    const _metaCount = response.data.metaBoard.metaCount;

    assert.equal(
      _metaCount,
      metaCount,
      `Incorrect metaCount ${metaCount} - ${metaCount}`
    );
  });
});
