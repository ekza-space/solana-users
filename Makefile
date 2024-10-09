clean:
	cargo clean && rm -rf .anchor

build:
	anchor build && anchor keys sync
test:
	anchor test --skip-local-validator