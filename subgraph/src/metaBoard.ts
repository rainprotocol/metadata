import { BigInt, Bytes } from "@graphprotocol/graph-ts";
import { MetaV1_2 as MetaV1Event } from "../generated/metaboard0/MetaBoard";
import { MetaBoard as MetaBoardContract } from "../generated/metaboard0/MetaBoard";
import { MetaBoard, MetaV1 } from "../generated/schema";

export function handleMetaV1_2(event: MetaV1Event): void {
  let metaBoard = MetaBoard.load(event.address);
  if ( !metaBoard ) {
    metaBoard = new MetaBoard(event.address);
    metaBoard.address = event.address;
    metaBoard.nextMetaId = BigInt.fromI32(0);
    metaBoard.save();
  }

  let metaV1 = new MetaV1(metaBoard.nextMetaId.toString());

  metaV1.metaBoard = metaBoard.address;

  metaV1.sender = event.params.sender;
  metaV1.subject = event.params.subject;

  // Use `try_hash` to safely attempt to get the metaHash
  let contract = MetaBoardContract.bind(event.address);
  let tryMetaHash = contract.try_hash(event.params.meta);

  if ( !tryMetaHash.reverted ) {
    metaV1.metaHash = tryMetaHash.value;
  } else {
    // Set a default value (e.g., zero bytes) if the hash call fails
    metaV1.metaHash = Bytes.fromHexString("0x00") as Bytes;
  }

  metaV1.meta = event.params.meta;

  metaV1.save();

  metaBoard.nextMetaId = metaBoard.nextMetaId.plus(BigInt.fromI32(1));
  metaBoard.save();
}