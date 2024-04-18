import { test, assert, createMockedFunction } from "matchstick-as";
import { handleMetaV1 } from "../src/metaBoard";
import { createNewMetaV1Event } from "./utils";
import { Bytes, BigInt, ethereum, Address } from "@graphprotocol/graph-ts";
import { MetaBoard } from "../generated/MetaBoard/MetaBoard";


test("Can mock metaBoard function correctly", () => {
  let contractAddress = Address.fromString("0x89205A3A3b2A69De6Dbf7f01ED13B2108B2c43e7");
  const hexString = "0x123456789abcde";
  const meta = Bytes.fromHexString(hexString);
  createMockedFunction(contractAddress, "hash", "hash(bytes):(bytes32)")
    .withArgs([ethereum.Value.fromBytes(meta)])
    .returns([ethereum.Value.fromBytes(meta)]);

  let metaBoardContract = MetaBoard.bind(contractAddress);
  let result = metaBoardContract.hash(meta);

  assert.equals(ethereum.Value.fromBytes(meta), ethereum.Value.fromBytes(result));
});

//
// test("MetaV1Event is handled", () => {
//   const address = "0xc0D477556c25C9d67E1f57245C7453DA776B51cf";
//   const subjectBigInt = BigInt.fromI32(123);
//   const hexString = "0x123456789abcde";
//   const bytes = Bytes.fromHexString(hexString);
//   let newMetaV1Event = createNewMetaV1Event(address, subjectBigInt, bytes);
//
//   handleMetaV1(newMetaV1Event);
//
//   // assert.fieldEquals("MetaV1", "1","sender", address)
//   assert.equals(ethereum.Value.fromString("1"), ethereum.Value.fromString("1"));
// });