# Solana Sandwich

A tiny `solana` `program` helper that allows you to prevent being `sandwiched` and hit with a malicious `instruction`.

## AllowRule

https://github.com/ohaddahan/solana-sandwich/blob/735fd57bba83a356e5975001d6ca8df414413eec/src/structs.rs#L4-L8

Enforce if you let an `instruction` from this `program` to run `before` or `after` your `instruction`.

[Some examples in tests](https://github.com/coral-xyz/anchor-by-example/blob/173e341825aa5731fdb6377dec5910503c84fa14/programs/sandwich/tests/sandwich.ts)
