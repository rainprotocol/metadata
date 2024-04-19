import { MetaV1 } from "../generated/MetaBoard/MetaBoard"; // Update the path as per your file structure
import { ethereum, Address, BigInt, Bytes } from "@graphprotocol/graph-ts";
import { newMockEvent } from "matchstick-as";
import {handleMetaV1} from "../src/metaBoard";


export function createNewMetaV1Event(sender: string, subject: BigInt, meta: Bytes): MetaV1 {
  // Create a mock ethereum.Event instance
  const metaV1Event = changetype<MetaV1>(newMockEvent());
  metaV1Event.parameters = new Array()
  metaV1Event.address = Address.fromString("0xfc34d331cc7a4b461b7ded2f6835a837411d8684")

  // let idParam = new ethereum.EventParam("id", ethereum.Value.fromI32(1));
  let senderParam = new ethereum.EventParam("sender", ethereum.Value.fromAddress(Address.fromString(sender)))

  let subjectParam = new ethereum.EventParam("subject", ethereum.Value.fromUnsignedBigInt(subject));
  let metaParam = new ethereum.EventParam("meta", ethereum.Value.fromBytes(meta));


  // metaV1Event.parameters.push(idParam);
  metaV1Event.parameters.push(senderParam);
  metaV1Event.parameters.push(subjectParam);
  metaV1Event.parameters.push(metaParam);
  return metaV1Event;
}

export function handleNewMetaV1Events(events: MetaV1[]): void {
  events.forEach(event => {
    handleMetaV1(event)
  })
}

export const CONTRACT_ADDRESS = Address.fromString("0xfc34d331cc7a4b461b7ded2f6835a837411d8684");
