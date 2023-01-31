import test from 'ava';

import fg from 'fast-glob';
import { glob, rsGlob, rustNativeGlob } from '../index.js';

async function getCase(glob1 = '*', glob2 = glob1) {
  const [fgRes, rgRes] = await Promise.all([
    fg(glob1, { dot: true, ignore: ['**/node_modules/**', '**/target/**', '.DS_Store'] }),
    rsGlob(glob2),
  ]);
  fgRes.sort();
  rgRes.sort();
  return [fgRes, rgRes];
}

test('glob-match result', (t) => {
  t.assert(glob('*', 'abc'));
  t.assert(glob('a/**/test', 'a/foo/bar/test'));
});
test('rs-glob result *', async (t) => {
  const [fgRes, rgRes] = await getCase('*');
  t.log(fgRes, rgRes);
  t.deepEqual(fgRes.length, rgRes.length);
  t.deepEqual(fgRes, rgRes);
});

test('rs-glob result package.json with git ls', async (t) => {
  const [fgRes, rgRes] = await getCase('**/package.json', '**/{package}.json');
  t.log(fgRes, rgRes);
  t.deepEqual(fgRes.length, rgRes.length);
  t.deepEqual(fgRes, rgRes);
});

test('rs-glob', (t) => {
  const res = rustNativeGlob(['**/package.json', '!**/node_modules/**']);
  t.log(res);
  t.assert(JSON.stringify(res));
});
