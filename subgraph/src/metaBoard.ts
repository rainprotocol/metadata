import { BigInt, json, log } from "@graphprotocol/graph-ts";
import { MetaV1 as MetaV1Event } from "../generated/MetaBoard/MetaBoard";
import { MetaBoard, MetaV1 } from "../generated/schema";
import { CBORDecoder } from "@rainprotocol/assemblyscript-cbor";

export function handleMetaV1(event: MetaV1Event): void {
  let metaBoard = MetaBoard.load(event.address);
  if (!metaBoard) {
    metaBoard = new MetaBoard(event.address);
    metaBoard.address = event.address;
    metaBoard.metaCount = BigInt.fromI32(0);
    metaBoard.save();
  }

  let metaData = event.params.meta.toHex().slice(18);
  let data = new CBORDecoder(stringToArrayBuffer(metaData));
  let jsonData = json.fromString(data.parse().stringify()).toObject();

  let metaV1 = new MetaV1(event.transaction.hash.toHex());
  metaV1.sender = event.params.sender;
  metaV1.meta = event.params.meta;
  metaV1.metaBoard = event.address;
  metaV1.subject = event.params.subject;

  metaV1.payload = jsonData.mustGet("0").toString();
  metaV1.magicNumber = jsonData.mustGet("1").toBigInt();
  metaV1.contentType = jsonData.mustGet("2").toString();
  metaV1.blockNumber = event.block.number;

  metaV1.save();

  metaBoard.metaCount = metaBoard.metaCount.plus(BigInt.fromI32(1));
  metaBoard.save();
}

function stringToArrayBuffer(val: string): ArrayBuffer {
  const buff = new ArrayBuffer(val.length / 2);
  const view = new DataView(buff);
  for (let i = 0, j = 0; i < val.length; i = i + 2, j++) {
    view.setUint8(j, u8(Number.parseInt(`${val.at(i)}${val.at(i + 1)}`, 16)));
  }
  return buff;
}
