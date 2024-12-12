# Advent of Code 2024


## Execute

Replace `aoc2024` with `cargo run --release --` if using Rust cargo.

```
Usage: aoc2024 <COMMAND>

Commands:
  run   
  all   
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

```
aoc2024 run --help
```
```
Usage: aoc2024 run [OPTIONS]

Options:
  -d, --day <DAY>    
  -p, --part <PART>  
  -h, --help         Print help
```

Run for specified day
```
aoc2024 run --day <DAY> --part <PART>
```

Run for a single day
```
aoc2024 run --day <DAY>
```

Run for latest day and specified part
```
aoc2024 run --part <PART>
```

Run for latest day and part
```
aoc2024 run
```

Run all days and parts
```
aoc2024 all
```

## Benchmark

Run all benchmarks
```
cargo bench
```

Run for a specific day
```
cargo bench "day <DAY>"
```

Run for a specific day and part
```
cargo bench "day <DAY> part <PART>"
```