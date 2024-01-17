# yabfr (yet another brainf\*\*k _in_ rust)

⚡**blazingly fast** brainf\*\*k interpreter made in rust

## usage

### wasm:
```sh
npm install yabfr
```

```ts
import { run } from "yabfr";

const output = run("brainf**k program");

console.log(output);
```

### rust
```rust
use yabfr::run;

let output = run("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.");

println!("{}", output)
```
