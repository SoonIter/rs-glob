import test from 'ava';

import fg from 'fast-glob';
import { glob, rsGlob } from '../index.js';

test('glob-match result', (t) => {
  t.assert(glob('*', 'abc'));
  t.assert(glob('a/**/test', 'a/foo/bar/test'));
});
test('rs-glob result', async (t) => {
  const [fgRes, rgRes] = await Promise.all([
    fg('*', { dot: true, ignore: ['**/node_modules/**', '**/target/**', '.DS_Store'] }),
    rsGlob('*'),
  ]);
  fgRes.sort();
  rgRes.sort();
  t.log(fgRes, rgRes);
  t.deepEqual(fgRes.length, rgRes.length);
  t.deepEqual(fgRes, rgRes);
});
