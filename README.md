# Advent of Code 2024


## Run

### Provide input

**Option 1:**

- Log in to https://adventofcode.com/
- Enter the network tab of the page inspector
- Refresh the page (F5) to see requests
- Select a request and look in Cookies tab
- Copy the `session` value
- Create `aoc_session_cookie.txt` and part the content

**Option 2:**

- Create `input` directory
- Create files in format `2024_day_<DAY>_input` for the desired days
- Copy the inputs from https://adventofcode.com/ into the appropriate folders

### Execute

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