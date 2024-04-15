import { BigInt } from "@graphprotocol/graph-ts";
import { MetaV1 as MetaV1Event } from "../generated/MetaBoard/MetaBoard";
import { MetaBoard as MetaBoardContract } from "../generated/MetaBoard/MetaBoard";
import { MetaBoard, MetaV1 } from "../generated/schema";

export function handleMetaV1(event: MetaV1Event): void {
  let metaBoard = MetaBoard.load(event.address);
  if ( !metaBoard ) {
    metaBoard = new MetaBoard(event.address);
    metaBoard.address = event.address;
    metaBoard.nextMetaId = BigInt.fromI32(0);
    metaBoard.save();
  }

  let metaV1 = new MetaV1(metaBoard.nextMetaId);

  metaV1.metaBoard = metaBoard.address;

  metaV1.sender = event.params.sender;
  metaV1.subject = event.params.subject;

  metaV1.metaHash = MetaBoardContract.bind(event.address).hash(event.params.meta);
  metaV1.meta = event.params.meta;

  metaV1.save();

  metaBoard.nextMetaId = metaBoard.nextMetaId.plus(BigInt.fromI32(1));
  metaBoard.save();
}