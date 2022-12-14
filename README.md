# Casper ZK with Risc Zero

This is the example of the `risc0` proof verification on Casper.
See [this blog post](https://odra.dev/blog/casper-zk-risc0) for full story.

## Install
You should install:
- just
- wabt
- [wasm-float-transpiler](https://github.com/chipshort/wasm-float-transpiler)
- [cargo-odra](https://github.com/odradev/cargo-odra)
- [casper-client](https://crates.io/crates/casper-client)

## Run
Generate the prove and test it against local Casper VM.
```bash
$ just run
```

## Use on Casper Livenet
First add your Casper keys to `keys` directory. 

Install the Verifier contract on chain.
```bash
$ just deploy-verifier
```

Obtain the contract hash (via integration.cspr.live) and call the `verify` method.
```bash
$ just call-verify hash-123...def
```
