import { test, assert, createMockedFunction, clearStore, describe, afterEach } from "matchstick-as";
import { handleMetaV1 } from "../src/metaBoard";
import { createNewMetaV1Event, CONTRACT_ADDRESS } from "./utils";
import { Bytes, BigInt, ethereum, Address } from "@graphprotocol/graph-ts";
import { MetaBoard } from "../generated/MetaBoard/MetaBoard";

const ENTITY_TYPE = "MetaV1";
test("Can mock metaBoard function correctly", () => {
  const hexString = "0x123456789abcde";
  const meta = Bytes.fromHexString(hexString);
  createMockedFunction(CONTRACT_ADDRESS, "hash", "hash(bytes):(bytes32)")
    .withArgs([ethereum.Value.fromBytes(meta)])
    .returns([ethereum.Value.fromBytes(meta)]);

  let metaBoardContract = MetaBoard.bind(CONTRACT_ADDRESS);
  let result = metaBoardContract.hash(meta);

  assert.equals(ethereum.Value.fromBytes(meta), ethereum.Value.fromBytes(result));
});
describe("Mocked Events", () => {
  afterEach(() => {
    clearStore();
  });

  test("Checks event params", () => {
    // Call mappings
    const address = "0xc0D477556c25C9d67E1f57245C7453DA776B51cf";
    const subjectBigInt = BigInt.fromI32(1000);
    const meta = Bytes.fromHexString("0x123456789abcde");
    let newMetaV1Event = createNewMetaV1Event(address, subjectBigInt, meta);

    handleMetaV1(newMetaV1Event);

    assert.entityCount(ENTITY_TYPE, 1);
    assert.equals(ethereum.Value.fromAddress(newMetaV1Event.address), ethereum.Value.fromAddress(CONTRACT_ADDRESS));
    assert.equals(ethereum.Value.fromUnsignedBigInt(newMetaV1Event.params.subject), ethereum.Value.fromUnsignedBigInt(subjectBigInt));
    assert.equals(ethereum.Value.fromBytes(newMetaV1Event.params.meta), ethereum.Value.fromBytes(meta));
  });

});

