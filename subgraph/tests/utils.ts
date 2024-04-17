import { MetaV1 } from "../generated/MetaBoard/MetaBoard"; // Update the path as per your file structure
import { ethereum, Address, BigInt, Bytes } from "@graphprotocol/graph-ts";
import { newMockEvent } from "matchstick-as";


export function createNewMetaV1Event(sender: Address, subject: BigInt, meta: Bytes): MetaV1 {
  // Create a mock ethereum.Event instance
  const metaV1Event = changetype<MetaV1>(newMockEvent());
  metaV1Event.parameters =
    [
      new ethereum.EventParam("sender", ethereum.Value.fromAddress(sender)),
      new ethereum.EventParam("sender", ethereum.Value.fromI32(subject)),
      new ethereum.EventParam("sender", ethereum.Value.fromBytes(meta))
    ];

  return metaV1Event;

}