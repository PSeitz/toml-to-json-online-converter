// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './pkg/hello_world';`
// will work here one day as well!
const toml2_json_web = import('./pkg/toml2_json_web');

toml2_json_web
  .then(m => m.greet('World!'))
  .catch(console.error);


async function oge(){
    let m = await toml2_json_web;
    return m.greet('World!')
}