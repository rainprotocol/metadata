import { BigInt, Bytes, json, log } from "@graphprotocol/graph-ts";
import { MetaV1 as MetaV1Event } from "../generated/MetaBoard/MetaBoard";
import { MetaBoard, MetaV1 } from "../generated/schema";
import { CBORDecoder } from "@rainprotocol/assemblyscript-cbor";
import { RAIN_META_DOCUMENT_HEX, getMetaV1 } from "./utils";
import { ContentMeta } from "./metaV1";

export function handleMetaV1(event: MetaV1Event): void {
  let metaBoard = MetaBoard.load(event.address);
  if (!metaBoard) {
    metaBoard = new MetaBoard(event.address);
    metaBoard.address = event.address;
    metaBoard.metaCount = BigInt.fromI32(0);
    metaBoard.save();
  }

  let meta = event.params.meta.toHex();
  if (meta.includes(RAIN_META_DOCUMENT_HEX)) {
    meta = meta.replace(RAIN_META_DOCUMENT_HEX, "");
    const data = new CBORDecoder(stringToArrayBuffer(meta));
    const res = data.parse();

    if (res.isObj) {
      const dataString = res.toString();
      const jsonObject = json.fromString(dataString);
      const jsonContent = jsonObject.toObject();
      if (ContentMeta.validate(jsonContent)) {
        let metaV1 = new MetaV1(event.params.meta);
        const payload = jsonContent.get("0");
        const magicNumber = jsonContent.get("1");
        const contentType = jsonContent.get("2");
        const contentEncoding = jsonContent.get("3");
        const contentLanguage = jsonContent.get("4");

        if (payload) {
          let auxPayload = payload.toString();
          if (auxPayload.startsWith("h'")) {
            auxPayload = auxPayload.replace("h'", "");
          }
          if (auxPayload.endsWith("'")) {
            auxPayload = auxPayload.replace("'", "");
          }

          metaV1.payload = Bytes.fromHexString(auxPayload);
        }

        if (magicNumber) metaV1.magicNumber = magicNumber.toBigInt();
        if (contentType) metaV1.contentType = contentType.toString();
        if (contentEncoding)
          metaV1.contentEncoding = contentEncoding.toString();
        if (contentLanguage)
          metaV1.contentLanguage = contentLanguage.toString();
        metaV1.sender = event.params.sender;
        metaV1.blockNumber = event.block.number;
        metaV1.meta = event.params.meta;
        metaV1.metaBoard = metaBoard.id;
        metaV1.subject = event.params.subject;
        metaV1.save();
      }
    } else if (res.isError) {
      log.warning("error in cbor decoding", []);
    } else if (res.isSequence) {
      log.warning("meta is sequence", []);
    }
  } else {
    log.warning("Not rain meta document {}", [meta]);
  }

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
