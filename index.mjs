import {shout} from "./target/jco/node_rust_wasm_lambda.js";

export async function handler(event) {
  console.log('EVENT', event)
  return shout(event.key1);
}
