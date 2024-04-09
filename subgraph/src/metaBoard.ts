import { BigInt, json } from "@graphprotocol/graph-ts";
import { MetaV1 as MetaV1Event } from "../generated/MetaBoard/MetaBoard";
import { MetaBoard as MetaBoardContract } from "../generated/MetaBoard/MetaBoard";
import { MetaBoard, MetaV1 } from "../generated/schema";
import { CBORDecoder } from "@rainprotocol/assemblyscript-cbor";

export function handleMetaV1(event: MetaV1Event): void {
  let metaBoard = MetaBoard.load(event.address);
  if ( !metaBoard ) {
    metaBoard = new MetaBoard(event.address);
    metaBoard.address = event.address;
    metaBoard.metaCount = BigInt.fromI32(0);
    metaBoard.save();
  }
  let metaData = event.params.meta.toHex().slice(18);

  let metaV1 = new MetaV1(event.transaction.hash.toHex());
  metaV1.sender = event.params.sender;
  metaV1.meta = event.params.meta;
  metaV1.metaHash = MetaBoardContract.bind(event.address).hash(event.params.meta);
  metaV1.metaBoard = event.address;
  metaV1.subject = event.params.subject;

  let data = new CBORDecoder(stringToArrayBuffer(metaData));
  let parsedData = data.parse();
  if ( parsedData.isObj ) {
    let jsonData = json.try_fromString(parsedData.stringify());
    if ( jsonData.isOk ) {
      let jsonDataArray = jsonData.value.toArray();
      if ( jsonDataArray.length ) {
        let payload = jsonDataArray[ 0 ].toObject().get("0");
        let magicNumber = jsonDataArray[ 0 ].toObject().get("1");
        let contentType = jsonDataArray[ 0 ].toObject().get("2");
        if ( payload && magicNumber && contentType ) {
          metaV1.payload = payload.toString();
          metaV1.magicNumber = magicNumber.toBigInt();
          metaV1.contentType = contentType.toString();
        }
      }
    }
  }

  metaV1.save();

  metaBoard.metaCount = metaBoard.metaCount.plus(BigInt.fromI32(1));
  metaBoard.save();

}

function stringToArrayBuffer(val: string): ArrayBuffer {
  const buff = new ArrayBuffer(val.length / 2);
  const view = new DataView(buff);
  for ( let i = 0, j = 0; i < val.length; i = i + 2, j++ ) {
    view.setUint8(j, u8(Number.parseInt(`${ val.at(i) }${ val.at(i + 1) }`, 16)));
  }
  return buff;
}
