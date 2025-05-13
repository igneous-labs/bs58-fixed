# bs58-fixed-wasm-tests

## Setup

```bash
bun install
```

## Run

Before running the tests, make sure the `wasm/test/consumer` rust crate has been rebuilt:

```bash
cd ../consumer
make
```

Then, run the tests

```bash
bun test
```
