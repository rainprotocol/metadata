import { Bytes, crypto } from "@graphprotocol/graph-ts";
import { MetaV1 } from "../generated/schema";

export const RAIN_META_DOCUMENT_HEX = "0xff0a89c674ee7874";

export function getKeccak256FromBytes(data_: Bytes): Bytes {
  return Bytes.fromByteArray(crypto.keccak256(Bytes.fromByteArray(data_)));
}

export function isHexadecimalString(str: string): boolean {
  // Check if string is empty
  if (str.length == 0) {
    return false;
  }

  // Check if each character is a valid hexadecimal character
  for (let i = 0; i < str.length; i++) {
    let charCode = str.charCodeAt(i);
    if (
      !(
        (charCode >= 48 && charCode <= 57) || // 0-9
        (charCode >= 65 && charCode <= 70) || // A-F
        (charCode >= 97 && charCode <= 102)
      )
    ) {
      // a-f
      return false;
    }
  }

  return true;
}

export function getMetaV1(meta_: Bytes): MetaV1 {
    const metaV1_ID = getKeccak256FromBytes(meta_);
  
    let metaV1 = MetaV1.load(metaV1_ID);
  
    if (!metaV1) {
      metaV1 = new MetaV1(metaV1_ID);
      metaV1.meta = meta_;
    }
  
    return metaV1;
  }