import { glob, rsGlob } from 'rs-glob';
import fg from 'fast-glob';
const { sync: fgSync } = fg;
console.log(glob('*', 'a'));
console.log(await rsGlob('*'));
console.log(fgSync('*'));
console.log(await fg('*'));
