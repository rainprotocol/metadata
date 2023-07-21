import {
    BigInt,
    Bytes,
    JSONValue,
    JSONValueKind,
    TypedMap,
  } from "@graphprotocol/graph-ts";
  import { getKeccak256FromBytes, isHexadecimalString } from "./utils";
  
  export class ContentMeta {
    rainMetaId: Bytes;
    payload: Bytes = Bytes.empty();
    magicNumber: BigInt = BigInt.zero();
    contentType: string = "";
    contentEncoding: string = "";
    contentLanguage: string = "";
  
    constructor(
      metaContentV1Object_: TypedMap<string, JSONValue>,
      rainMetaID_: Bytes
    ) {
      const payload = metaContentV1Object_.get("0");
      const magicNumber = metaContentV1Object_.get("1");
      const contentType = metaContentV1Object_.get("2");
      const contentEncoding = metaContentV1Object_.get("3");
      const contentLanguage = metaContentV1Object_.get("4");
  
      // RainMetaV1 ID
      this.rainMetaId = rainMetaID_;
  
      // Mandatories keys
      if (payload) {
        let auxPayload = payload.toString();
        if (auxPayload.startsWith("h'")) {
          auxPayload = auxPayload.replace("h'", "");
        }
        if (auxPayload.endsWith("'")) {
          auxPayload = auxPayload.replace("'", "");
        }
  
        this.payload = Bytes.fromHexString(auxPayload);
      }
  
      // if (payload) this.payload = payload.toString();
      if (magicNumber) this.magicNumber = magicNumber.toBigInt();
  
      // Keys optionals
      if (contentType) this.contentType = contentType.toString();
      if (contentEncoding) this.contentEncoding = contentEncoding.toString();
      if (contentLanguage) this.contentLanguage = contentLanguage.toString();
    }
  
    /**
     * Validate that the keys exist on the map
     */
    static validate(metaContentV1Object: TypedMap<string, JSONValue>): boolean {
      const payload = metaContentV1Object.get("0");
      const magicNumber = metaContentV1Object.get("1");
      const contentType = metaContentV1Object.get("2");
      const contentEncoding = metaContentV1Object.get("3");
      const contentLanguage = metaContentV1Object.get("4");
  
      // Only payload and magicNumber are mandatory on RainMetaV1
      // See: https://github.com/rainprotocol/specs/blob/main/metadata-v1.md
      if (payload && magicNumber) {
        if (
          payload.kind == JSONValueKind.STRING ||
          magicNumber.kind == JSONValueKind.NUMBER
        ) {
          // Check if payload is a valid Bytes (hexa)
          let auxPayload = payload.toString();
          if (auxPayload.startsWith("h'")) {
            auxPayload = auxPayload.replace("h'", "");
          }
          if (auxPayload.endsWith("'")) {
            auxPayload = auxPayload.replace("'", "");
          }
  
          // If the payload is not a valid bytes value
          if (!isHexadecimalString(auxPayload)) {
            return false;
          }
  
          // Check the type of optionals keys
          if (contentType) {
            if (contentType.kind != JSONValueKind.STRING) {
              return false;
            }
          }
          if (contentEncoding) {
            if (contentEncoding.kind != JSONValueKind.STRING) {
              return false;
            }
          }
          if (contentLanguage) {
            if (contentLanguage.kind != JSONValueKind.STRING) {
              return false;
            }
          }
  
          return true;
        }
      }
  
      return false;
    }
  
    private getContentId(): Bytes {
      // Values as Bytes
      const payloadB = this.payload;
      const magicNumberB = Bytes.fromHexString(this.magicNumber.toHex());
      const contentTypeB = Bytes.fromUTF8(this.contentType);
      const contentEncodingB = Bytes.fromUTF8(this.contentEncoding);
      const contentLanguageB = Bytes.fromUTF8(this.contentLanguage);
  
      // payload +  magicNumber + contentType + contentEncoding + contentLanguage
      const contentId = getKeccak256FromBytes(
        payloadB
          .concat(magicNumberB)
          .concat(contentTypeB)
          .concat(contentEncodingB)
          .concat(contentLanguageB)
      );
  
      return contentId;
    }
  
    /**
     * Create or generate a MetaContentV1 entity based on the current fields:
     *
     * - If the MetaContentV1 does not exist, create the MetaContentV1 entity and
     * made the relation to the rainMetaId.
     *
     * - If the MetaContentV1 does exist, add the relation to the rainMetaId.
     */
    // generate(): MetaContentV1 {
    //   const contentId = this.getContentId();
  
    //   let metaContent = MetaContentV1.load(contentId);
  
    //   if (!metaContent) {
    //     metaContent = new MetaContentV1(contentId);
  
    //     metaContent.payload = this.payload;
    //     metaContent.magicNumber = this.magicNumber;
    //     metaContent.documents = [];
  
    //     if (this.contentType != "") metaContent.contentType = this.contentType;
  
    //     if (this.contentEncoding != "")
    //       metaContent.contentEncoding = this.contentEncoding;
  
    //     if (this.contentLanguage != "")
    //       metaContent.contentLanguage = this.contentLanguage;
    //   }
  
    //   const aux = metaContent.documents;
    //   if (!aux.includes(this.rainMetaId)) aux.push(this.rainMetaId);
  
    //   metaContent.documents = aux;
  
    //   metaContent.save();
  
    //   return metaContent;
    // }
  }