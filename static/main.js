import init, { run_app } from './package.js';
async function main() {
   await init('/package_bg.wasm');
   run_app();
}
main()
