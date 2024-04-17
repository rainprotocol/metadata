import {strict as assert} from "assert";
import {MetaBoard} from '../generated/MetaBoard/MetaBoard'

describe('MetaBoard', () => {
    it('should hash the meta correctly', () => {
        let hashed = MetaBoard.hash("0xffa")
      assert.equal(hashed, 'asd')
    });
});
