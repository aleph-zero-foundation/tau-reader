# Tau reader

A simple library + binary to read trusted setup from the AWS S3 bucket.
By default, it also validates fetched values (against known tau).

After running `cargo run --release`, you should see something like this:
```
Getting points for the power: `3542`... ✅
Getting points for the power: `3602`... ✅
Getting points for the power: `3650`... ✅
Getting points for the power: `2440`... ✅
Getting points for the power: `2101`... ✅
...
Getting points for the power: `8733`... ✅
Getting points for the power: `3776`... ✅
Getting points for the power: `5660`... ✅
Getting points for the power: `1621`... ✅
Getting points for the power: `7215`... ✅
Getting points for the power: `53105317583`... ✅
Getting points for the power: `34729604171`... ✅
Getting points for the power: `44777395951`... ✅
Getting points for the power: `64057108245`... ✅
Getting points for the power: `55537233474`... ✅
...
Getting points for the power: `14463773455`... ✅
Getting points for the power: `36886839432`... ✅
Getting points for the power: `13361538728`... ✅
Getting points for the power: `41976548248`... ✅
Getting points for the power: `26547567997`... ✅
```
