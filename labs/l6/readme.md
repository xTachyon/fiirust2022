# Recap

Sorting vectors:
```rs
let mut v = vec!["green", "red"];
v.push("yellow");
v.sort_unstable_by_key(|x| x.len());
v.sort_unstable_by(|x, y| x.len().cmp(&y.len())); // same as before
```
Maps:
```rs
let mut map = HashMap::new();

map.insert("a", 5);
map.entry("b").or_default(); // insert ("b", 0) because it doesn't exist
map.entry("b").or_insert(5); // does nothing because "b" already exists
map.entry("c").and_modify(|x| *x += 2).or_insert(1); // if "c" exists, add 2 to the value, else insert 1
```

# Formatting to strings example
```rs
use std::fmt::Write;

let mut s = String::new();
writeln!(&mut s, "something: {}", 5)?;
write!(&mut s, "something else: {} ", 6)?;
write!(&mut s, "something else: {} ", 7)?;
writeln!(&mut s)?;
```

# Problems

## P1

Read a text file and split it into words (separators: [whitespaces](https://doc.rust-lang.org/std/primitive.char.html#method.is_ascii_whitespace), [punctuation](https://doc.rust-lang.org/std/primitive.char.html#method.is_ascii_punctuation)). Words are case insensitive. Print the data sorted by the number of apparitions, in the format below.

Example input:
```
I bought an apple. Then I eat an apple. Apple is my favorite.
```
Example output:
```
apple    => 3
an       => 2
i        => 2
eat      => 1
bought   => 1
is       => 1
favorite => 1
my       => 1
then     => 1
```
Notice that the output is aligned so that the count is on the same column. Use a map with the entry API to add elements to the map.

## Bonus

[clap](https://crates.io/crates/clap) is a crate for parsing command line arguments into Rust types that can be more easily processed afterwards.

`Cargo.toml`:
```toml
clap = { version = "4", features = ["derive"] }
```

The convention for arguments is as following: subcommands will be spelled as-in (ex: `run`), short arguments are spelled with a single letter and with a dash (ex: `-f`), and long arguments are spelled as-is with two dashes (ex: `--file`). Usually, an argument is by default considered long, with an optional short argument that does the same thing (ex: `-f` and `--file` can be the same). A single `--` usually means that everything past this point will be passed to the application that will be invoked next.

Example code (modified from the [docs](https://docs.rs/clap/latest/clap/#example)):
```rs
use clap::Parser;

#[derive(Parser)]
#[command(version, about = "args parsing example")]
struct Args {
   /// Name of the person to greet
   #[arg(short, long, default_value = "you 👀")]
   name: String,

   /// Number of times to greet
   #[arg(short, long, default_value_t = 1)]
   count: u8,
}

fn main() {
   let args = Args::parse();

   for _ in 0..args.count {
       println!("Hello {}!", args.name)
   }
}
```
Arguments are usually parsed into a struct, but it's also possible to parse them into enum, especially when we're working with subcommands.

```rs
#[command(version, about = "args parsing example")]
```
In this example, we tell `clap` that:
- we want to generate a version command (that by default prints the version from `Cargo.toml`)
- we give it a string to print about what the application does (in the help command)

```rs
/// Number of times to greet
#[arg(short, long, default_value_t = 1)]
count: u8,
```
We tell it:
- the description of the command that will be printed in help
- that we want short and long command versions
- if the command is not found, then the default value is 1. Note that we could've done the same thing with an `Option<u8>`

After this, we can use arguments as a normal Rust struct. The `parse` functionw will exit the application if the parsing triggered any error.

Invocation example:
```
./target/debug/hello_world.exe -h
./target/debug/hello_world.exe -nDragos -c 4
```
To run it through `cargo`, we need to pass it `--`, and then the arguments for our app:
```
cargo run -- -nDragos -c 4
cargo run -- --name Kratos -c2
```

## P2
Go back to P1 and implement a command line interface that can support the following args:
- file: the name of the file to print the result to. If empty, print to stdout
- print_max: the maximum number of entries to print at the end. If empty, print all of them