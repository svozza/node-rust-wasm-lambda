# Node.js and WASM Lambda

## Introduction

This is a sample project for compiling a Rust module to WASM and using JCO to import it in a Node.js 
AWS Lambda function.

## Prerequisites

You must have the following tooling installed

- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/en/download/package-manager/current)
- [cargo-component](https://github.com/bytecodealliance/cargo-component)

## Setup

Note: This is based on the JCO launch announcement [blog post](https://bytecodealliance.org/articles/jco-1.0) but 
the steps appear to be outdated so what follows is the changes required to make it work as of 2024-05-25.

The project was created by running the following `cargo-component` command:

```shell
cargo component new <project-name> --lib
```
This creates a component in `src/lib.rs` that has a simple Hello World function. This function was changed to one 
that will turn a string into upper case:

```rust
impl Guest for Component {
    fn shout(input: String) -> String {
        let mut s = input.to_uppercase();
        s.push_str("!");
        s.into()
    }
}
```

The `wit/world.wit` file which uses the [IDL(https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md)] 
that underpins the WebAssembly component model was also changed:

```
package component:<project-name>;

world example {
    export shout: func(input: string) -> string;
}
```

## Instructions:
1. Run `npm i` to install the `@bytecodealliance/jco`  and`@bytecodealliance/preview2-shim` dependencies 
aqqqlocally.
2. Run `npm run build` to compile the Rust module, invoke JCO and then build a zip file of the code
in a folder called `dist`.
3. Create a blank Node.js AWS lambda function.
4. Upload the zip to the lambda function.
5. Execute the lambda function with the default test event, which should produce the following output:

```
"VALUE1!"
```