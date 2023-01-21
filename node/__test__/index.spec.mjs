import test from 'ava';

import { sum, glob } from '../index.js';

test('sum from native', t => {
  t.is(sum(1, 2), 3);
  console.log(sum(1, 2));
});

test('nr native', t => {
  t.assert(glob('*', 'abc'));
  t.assert(glob("a/**/test", "a/foo/bar/test"));
});
