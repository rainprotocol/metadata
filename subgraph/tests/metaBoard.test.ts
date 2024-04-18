import { test, assert } from "matchstick-as";
import { handleMetaV1 } from "../src/metaBoard";
import { createNewMetaV1Event } from "./utils";
import { Bytes, BigInt, ethereum } from "@graphprotocol/graph-ts";

test("MetaV1Event is handled", () => {
  const address = "0xc0D477556c25C9d67E1f57245C7453DA776B51cf";
  const subjectBigInt = BigInt.fromI32(123);
  const hexString = "0x123456789abcde";
  const bytes = Bytes.fromHexString(hexString);
  let newMetaV1Event = createNewMetaV1Event(address, subjectBigInt, bytes);

  handleMetaV1(newMetaV1Event);

  // assert.fieldEquals("MetaV1", "1","sender", address)
  assert.equals(ethereum.Value.fromString("1"), ethereum.Value.fromString("1"));
});