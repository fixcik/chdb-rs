# Chdb for rust

Use clickhouse as library, based on `clickhouse local`

### Requirements:

You should have [libchdb](https://github.com/metrico/libchdb#package-installation)

### Install

```
cargo add chdb
```
or add to Cargo.toml
```
chdb = "0.1"
```

Powered by:

- ClickHouse - https://clickhouse.com/
- Chdb - https://github.com/chdb-io/chdb
- libchdb - https://github.com/metrico/libchdb



## Usage

```rust
use chdb::{flag, option, Query};

let result = Query::new("SELECT number FROM numbers(10)")
    .option(option!("format", "TSVWithNames"))
    .option(flag!("verbose"))
    .exec()
    .unwrap();

println!("Elapsed: {}", result.elapsed);
println!("Rows: {}", result.rows_read);
println!("Bytes: {}", result.bytes_read);
println!("Result:\n{}", result.to_string().unwrap());
```

Otputs:
```
SELECT number FROM numbers(10)
Elapsed: 0.007413874
Rows: 10
Bytes: 80
Result:
number
0
1
2
3
4
5
6
7
8
9

```

## Options and flags

List of available options [here](OPTIONS.md)
