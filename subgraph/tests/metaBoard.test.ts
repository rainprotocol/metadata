import { test, assert, createMockedFunction, clearStore, describe, afterEach } from "matchstick-as";
import { handleMetaV1 } from "../src/metaBoard";
import { createNewMetaV1Event, CONTRACT_ADDRESS } from "./utils";
import { Bytes, BigInt, ethereum, Address } from "@graphprotocol/graph-ts";
import { MetaBoard } from "../generated/MetaBoard/MetaBoard";

const ENTITY_TYPE_META_V1 = "MetaV1";
const ENTITY_TYPE_META_BOARD = "MetaBoard";
test("Can mock metaBoard function correctly", () => {
  const meta = Bytes.fromHexString("0xff0a89c674ee7874010203");
  createMockedFunction(CONTRACT_ADDRESS, "hash", "hash(bytes):(bytes32)")
    .withArgs([ethereum.Value.fromBytes(meta)])
    .returns([ethereum.Value.fromBytes(Bytes.fromHexString("0x6bdf81f785b54fd65ca6fc5d02b40fa361bc7d5f4f1067fc534b9433ecbc784d"))]);

  let metaBoardContract = MetaBoard.bind(CONTRACT_ADDRESS);
  let result = metaBoardContract.hash(meta);

  assert.equals(ethereum.Value.fromBytes(Bytes.fromHexString("0x6bdf81f785b54fd65ca6fc5d02b40fa361bc7d5f4f1067fc534b9433ecbc784d")), ethereum.Value.fromBytes(result));
});
describe("Mocked Events", () => {
  afterEach(() => {
    clearStore();
  });

  test("Checks event params", () => {
    // Call mappings
    const sender = "0xc0D477556c25C9d67E1f57245C7453DA776B51cf";
    const subjectBigInt = BigInt.fromI32(1000);
    const meta = Bytes.fromHexString("0x123456789abcde");
    let newMetaV1Event = createNewMetaV1Event(sender, subjectBigInt, meta);

    handleMetaV1(newMetaV1Event);

    assert.entityCount(ENTITY_TYPE_META_V1, 1);
    assert.addressEquals(newMetaV1Event.address, CONTRACT_ADDRESS);
    assert.equals(ethereum.Value.fromUnsignedBigInt(newMetaV1Event.params.subject), ethereum.Value.fromUnsignedBigInt(subjectBigInt));
    assert.equals(ethereum.Value.fromBytes(newMetaV1Event.params.meta), ethereum.Value.fromBytes(meta));
  });
  //
  // test("Checks entity count", () => {
  //   // Call mappings
  //   let firstMetaV1Event = createNewMetaV1Event(
  //     "0xc0D477556c25C9d67E1f57245C7453DA776B51cf",
  //     BigInt.fromI32(1000),
  //     Bytes.fromHexString("0x123456789abcde"));
  //   let secondMetaV1Event = createNewMetaV1Event(
  //     "0xc0D477556c25C9d67E1f57245C7453DA776B51cf",
  //     BigInt.fromI32(2000),
  //     Bytes.fromHexString("0x1234"));
  //   let thirdMetaV1Event = createNewMetaV1Event(
  //     "0xc0D477556c25C9d67E1f57245C7453DA776B51cf",
  //     BigInt.fromI32(3000),
  //     Bytes.fromHexString("0x1456"));
  //
  //
  //   handleNewMetaV1Events([firstMetaV1Event, secondMetaV1Event,thirdMetaV1Event]);
  //   assert.entityCount(ENTITY_TYPE_META_V1, 3);
  //   // assert.addressEquals(newMetaV1Event.address, CONTRACT_ADDRESS);
  //   // assert.equals(ethereum.Value.fromUnsignedBigInt(newMetaV1Event.params.subject), ethereum.Value.fromUnsignedBigInt(subjectBigInt));
  //   // assert.equals(ethereum.Value.fromBytes(newMetaV1Event.params.meta), ethereum.Value.fromBytes(meta));
  // });

});

