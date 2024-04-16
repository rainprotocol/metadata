import { strict as assert } from "assert";
import { BigInt, ethereum } from'@graphprotocol/graph-ts';
import { handleMetaV1 } from'../src/metaBoard';
import { MetaV1 } from'../generated/schema';

describe('MetaV1 Event Handler', () => {
  it('should handle MetaV1 event correctly', () => {
    const event = new ethereum.Event();
    event.address = '0x123';
    event.params = {
      sender: '0x456',
      subject: 'Some subject',
      meta: 'Some meta data',
    };

    const loadMock = (address) => {
      if (address === event.address) {
        return {
          address: event.address,
          nextMetaId: BigInt.fromI32(0),
          save: () => {},
        };
      }
      return null;
    };

    const bindMock = () => ({
      hash: () => 'Mock hash',
    });

    const plusMock = () => BigInt.fromI32(1);

    const originalLoad = MetaBoard.load;
    const originalBind = MetaBoard.bind;
    const originalBigIntPlus = BigInt.plus;

    MetaBoard.load = loadMock;
    MetaBoard.bind = bindMock;
    BigInt.plus = plusMock;

    handleMetaV1(event);


    const metaV1 = MetaV1.load('0'); // Assuming nextMetaId starts from '0'
    assert.should.exist(metaV1)
    assert.equal(metaV1.metaBoard, event.address);
    assert.equal(metaV1.sender, event.params.sender);
    assert.equal(metaV1.subject, event.params.subject);
    assert.equal(metaV1.metaHash, 'Mock hash');
    assert.equal(metaV1.meta, event.params.meta);

    // Restore the original implementations
    MetaBoard.load = originalLoad;
    MetaBoard.bind = originalBind;
    BigInt.plus = originalBigIntPlus;
  });
});
