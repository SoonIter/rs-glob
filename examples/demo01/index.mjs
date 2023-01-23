import { glob, rsGlob } from 'rs-glob';
import fg from 'fast-glob';
const { sync: fgSync } = fg;
console.log(glob('*', 'a'));
console.log(rsGlob('*'));
console.log(fgSync('*'));
