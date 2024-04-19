import { test, assert, createMockedFunction, clearStore, describe, afterEach} from "matchstick-as";
import { handleMetaV1 } from "../src/metaBoard";
import { createNewMetaV1Event, handleNewMetaV1Events } from "./utils";
import { Bytes, BigInt, ethereum, Address } from "@graphprotocol/graph-ts";
import { MetaBoard } from "../generated/MetaBoard/MetaBoard";

let contractAddress = Address.fromString("0x89205A3A3b2A69De6Dbf7f01ED13B2108B2c43e7");

test("Can mock metaBoard function correctly", () => {
  const hexString = "0x123456789abcde";
  const meta = Bytes.fromHexString(hexString);
  createMockedFunction(contractAddress, "hash", "hash(bytes):(bytes32)")
    .withArgs([ethereum.Value.fromBytes(meta)])
    .returns([ethereum.Value.fromBytes(meta)]);

  let metaBoardContract = MetaBoard.bind(contractAddress);
  let result = metaBoardContract.hash(meta);

  assert.equals(ethereum.Value.fromBytes(meta), ethereum.Value.fromBytes(result));
});
describe("Mocked Events", () => {
  afterEach(() => {
    clearStore()
  })

  test("Can call mappings with custom events", () => {
    // Call mappings
  const address = "0xc0D477556c25C9d67E1f57245C7453DA776B51cf";
  const subjectBigInt = BigInt.fromI32(123);
  const hexString = "0x123456789abcde";
  const bytes = Bytes.fromHexString(hexString);
  let newMetaV1Event = createNewMetaV1Event(address, subjectBigInt, bytes);
    const meta = Bytes.fromHexString(hexString);
    createMockedFunction(contractAddress, "hash", "hash(bytes):(bytes32)")
      .withArgs([ethereum.Value.fromBytes(meta)])
      .returns([ethereum.Value.fromBytes(meta)]);


    let anotherMetaV1Event = createNewMetaV1Event(
      "0x89205A3A3b2A69De6Dbf7f01ED13B2108B2c43e7",
      BigInt.fromI32(456),
      bytes,
    )

    handleNewMetaV1Events([newMetaV1Event, anotherMetaV1Event])

    // assert.entityCount(GRAVATAR_ENTITY_TYPE, 2)
    // assert.fieldEquals(GRAVATAR_ENTITY_TYPE, "0xdead", "displayName", "Gravatar 0xdead")
    // assert.fieldEquals(GRAVATAR_ENTITY_TYPE, "0xbeef", "displayName", "Gravatar 0xbeef")
  })

})

