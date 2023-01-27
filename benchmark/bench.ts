import b from 'benny';
import { rsGlob } from 'rs-glob';
import fg from 'fast-glob';
const { sync: fgSync } = fg;
b.suite(
  'overwrite',
  b.add('fast-glob async', async () => {
    await fg('**/package.json', { dot: true, ignore: ['**/node_modules/**', '**/target/**', '.DS_Store'] });
  }),
  b.add('no ignore', async () => {
    await fg('**/package.json');
  }),
  b.add('fast-glob sync', () => {
    fgSync('**/package.json', { dot: true, ignore: ['**/node_modules/**', '**/target/**', '.DS_Store'] });
  }),
  b.add('rs-glob async', async () => {
    await rsGlob('**/package.json');
  }),

  b.cycle(),
  b.complete(),
);

// async function hello() {
//   const arr1 = await fg('**/package.json', { dot: false, ignore: ['**/node_modules/**', '**/target/**', '.DS_Store'] });
//   const arr2 = await rsGlob('**/package.json');

//   console.log(arr1, arr2);
// }
// hello();
