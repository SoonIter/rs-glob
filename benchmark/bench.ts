import b from 'benny';
import { rsGlob, rustNativeGlob } from 'rs-glob';
import fg from 'fast-glob';
const { sync: fgSync } = fg;
b.suite(
  'overwrite',
  b.add('fast-glob async', async () => {
    await fg('**/package.json', { dot: true, ignore: ['**/node_modules', '**/dist'] });
  }),
  b.add('fast-glob sync', () => {
    fgSync('**/package.json', { dot: true, ignore: ['**/node_modules', '**/dist'] });
  }),
  b.add('fast-glob no ignore node_modules', async () => {
    await fg('**/package.json');
  }),
  b.add('rs-glob async with git ls', async () => {
    await rsGlob('**/package.json');
  }),
  b.add('rs-glob dir', () => {
    rustNativeGlob(['**/package.json', '!**/node_modules', '!**/dist']);
  }),
  b.cycle(),
  b.complete(),
);

async function hello() {
  const arr1 = await fg('**/package.json', { dot: false, ignore: ['**/node_modules', '**/dist'] });
  const arr2 = await rsGlob('**/package.json');
  const arr3 = rustNativeGlob(['**/package.json', '!**/node_modules', '!**/dist']);
  arr1.sort();
  arr2.sort();
  arr3.sort();

  console.log(arr1, arr2, arr3);
  console.log(arr1.length, arr2.length, arr3.length, arr1.length === arr2.length && arr2.length === arr3.length);
}
hello();
