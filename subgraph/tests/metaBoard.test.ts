import {
  test,
  assert,
  createMockedFunction,
  clearStore,
  describe,
  afterEach,
  beforeAll,
  afterAll,
  newMockEvent,
  clearInBlockStore
} from "matchstick-as";
import { createNewMetaV1Event, CONTRACT_ADDRESS } from "./utils";
import { Bytes, BigInt, ethereum, Address } from "@graphprotocol/graph-ts";
import { MetaBoard as MetaBoardContract, MetaV1 } from "../generated/metaboard0/MetaBoard";
import { MetaBoard, MetaV1 as MetaV1Entity } from "../generated/schema";
import { handleMetaV1 } from "../src/metaBoard";

const ENTITY_TYPE_META_V1 = "MetaV1";
const ENTITY_TYPE_META_BOARD = "MetaBoard";

describe("Test meta event", () => {
  afterEach(() => {
    clearStore();
    clearInBlockStore();
  });
  test("Can mock metaBoard function correctly", () => {
    const meta = Bytes.fromHexString("0xff0a89c674ee7874010203");
    createMockedFunction(CONTRACT_ADDRESS, "hash", "hash(bytes):(bytes32)")
      .withArgs([ethereum.Value.fromBytes(meta)])
      .returns([ethereum.Value.fromBytes(Bytes.fromHexString("0x6bdf81f785b54fd65ca6fc5d02b40fa361bc7d5f4f1067fc534b9433ecbc784d"))]);

    let metaBoardContract = MetaBoardContract.bind(CONTRACT_ADDRESS);
    let result = metaBoardContract.hash(meta);

    assert.equals(ethereum.Value.fromBytes(Bytes.fromHexString("0x6bdf81f785b54fd65ca6fc5d02b40fa361bc7d5f4f1067fc534b9433ecbc784d")), ethereum.Value.fromBytes(result));
  });
  test("Checks event params", () => {
    // Call mappings
    const sender = "0xc0D477556c25C9d67E1f57245C7453DA776B51cf";
    const subjectBigInt = BigInt.fromI32(1000);
    const meta = Bytes.fromHexString("0xff0a89c674ee7874010203");
    let newMetaV1Event = createNewMetaV1Event(sender, subjectBigInt, meta);

    createMockedFunction(CONTRACT_ADDRESS, "hash", "hash(bytes):(bytes32)")
      .withArgs([ethereum.Value.fromBytes(meta)])
      .returns([ethereum.Value.fromBytes(Bytes.fromHexString("0x6bdf81f785b54fd65ca6fc5d02b40fa361bc7d5f4f1067fc534b9433ecbc784d"))]);

    handleMetaV1(newMetaV1Event);

    assert.entityCount(ENTITY_TYPE_META_V1, 1);
    assert.addressEquals(newMetaV1Event.address, CONTRACT_ADDRESS);
    assert.equals(ethereum.Value.fromUnsignedBigInt(newMetaV1Event.params.subject), ethereum.Value.fromUnsignedBigInt(subjectBigInt));
    assert.equals(ethereum.Value.fromBytes(newMetaV1Event.params.meta), ethereum.Value.fromBytes(meta));
  });
  test("Can update event metadata", () => {
    const metaV1Event = changetype<MetaV1>(newMockEvent());
    metaV1Event.parameters = new Array();

    const sender = "0x8058ad7C22fdC8788fe4cB1dAc15D6e976127324";
    const subjectBigInt = BigInt.fromI32(2000);
    const meta = Bytes.fromHexString("0xff0a89c674ee787401020304");

    let UPDATED_SENDER = new ethereum.EventParam("sender", ethereum.Value.fromAddress(Address.fromString(sender)));
    let UPDATED_SUBJECT = new ethereum.EventParam("subject", ethereum.Value.fromUnsignedBigInt(subjectBigInt));
    let UPDATED_META = new ethereum.EventParam("meta", ethereum.Value.fromBytes(meta));

    metaV1Event.parameters.push(UPDATED_SENDER);
    metaV1Event.parameters.push(UPDATED_SUBJECT);
    metaV1Event.parameters.push(UPDATED_META);

    assert.addressEquals(Address.fromString(sender), metaV1Event.params.sender);
    assert.bigIntEquals(subjectBigInt, metaV1Event.params.subject);
    assert.bytesEquals(meta, metaV1Event.params.meta);
  });
  test("Returns null when calling entity.load() if an entity doesn't exist", () => {
    let retrievedMetaV1 = MetaV1Entity.load("1");
    assert.assertNull(retrievedMetaV1);
  });

});

describe("Test Metaboard Entity", () => {
  beforeAll(() => {
    const sender = "0xc0D477556c25C9d67E1f57245C7453DA776B51cf";
    const subjectBigInt = BigInt.fromI32(1000);
    const meta = Bytes.fromHexString("0xff0a89c674ee7874010203");
    let newMetaV1Event = createNewMetaV1Event(sender, subjectBigInt, meta);

    createMockedFunction(CONTRACT_ADDRESS, "hash", "hash(bytes):(bytes32)")
      .withArgs([ethereum.Value.fromBytes(meta)])
      .returns([ethereum.Value.fromBytes(Bytes.fromHexString("0x6bdf81f785b54fd65ca6fc5d02b40fa361bc7d5f4f1067fc534b9433ecbc784d"))]);

    handleMetaV1(newMetaV1Event);

  });

  afterAll(() => {
    clearStore();
    clearInBlockStore();
  });

  test("Checks MetaBoard entity", () => {
    let retrievedMetaBoard = MetaBoard.load(CONTRACT_ADDRESS) as MetaBoard;
    assert.entityCount(ENTITY_TYPE_META_BOARD, 1);
    assert.addressEquals(Address.fromBytes(retrievedMetaBoard.address), CONTRACT_ADDRESS);
  });
  test("Returns null when calling entity.loadInBlock() if an entity doesn't exist in the current block", () => {
    let retrievedMetaBoard = MetaBoard.loadInBlock(Address.fromString("0x33F77e7Bc935503e437166498D7D72f2Ea290E1f"));
    assert.assertNull(retrievedMetaBoard);
  });

});

