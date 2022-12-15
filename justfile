WASM_FILE := "verifier/wasm/verifier.wasm"
CASPER_NODE_ADDRESS := "http://3.140.179.157:7777"
CASPER_NETWORK_NAME := "integration-test"
SECRET_KEY := "keys/secret_key.pem"

prove:
    cd prover && cargo run --release

build:
    cd verifier && cargo odra build -b casper

test-mock-vm:
    cd verifier && cargo odra test

only-test:
    cd verifier && cargo odra test -b casper --skip-build

test: build only-test

run: prove build test

deploy-verifier:
    casper-client put-deploy \
        --node-address {{CASPER_NODE_ADDRESS}} \
        --chain-name {{CASPER_NETWORK_NAME}} \
        --secret-key {{SECRET_KEY}} \
        --session-path {{WASM_FILE}} \
        --payment-amount 500000000000

call-verify VERIFIER_HASH:
    casper-client put-deploy \
        --node-address {{CASPER_NODE_ADDRESS}} \
        --chain-name {{CASPER_NETWORK_NAME}} \
        --secret-key {{SECRET_KEY}} \
        --session-hash {{VERIFIER_HASH}} \
        --session-entry-point "verify" \
        --payment-amount 5000000000000

clean:
    cd prover && cargo clean
    cd verifier && cargo odra clean
    cd methods && cargo clean