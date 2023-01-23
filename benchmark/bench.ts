import b from 'benny';
import { glob, rsGlob } from 'rs-glob';
import fg from 'fast-glob';
const { sync: fgSync } = fg;
b.suite(
  'overwrite',
  b.add('fast-glob', () => {
    const arr1 = fgSync('**/package.json', { dot: true, ignore: ['**/node_modules/**', '**/target/**'] });
  }),
  b.add('rs-glob', () => {
    const arr2 = rsGlob('**/package.json');
  }),
  b.cycle(),
  b.complete(),
);
