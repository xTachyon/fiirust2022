# Recap

Copying files:
```rs
use std::fs::File;
use std::io::{Read, Write};

fn main() -> Result<(), std::io::Error> {
    let path_in = "in.png";
    let path_out = "out.png";

    let mut file_in = File::open(path_in)?;
    let mut file_out = File::create(path_out)?;
    let mut buffer = [0; 4096];
    loop {
        let read = file_in.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        file_out.write_all(&buffer[..read])?;
    }

    Ok(())
}
```

Reading arguments:
```rs
use std::env;

fn main() {
    for argument in env::args() {
        println!("{argument}");
    }
}
```

# Problems

## P1

Read a binary file in a buffered way (`BufReader` or reading it in chunks), and transform it to a hex (base 16) string that you write to another file. Test it with an image. Decode it with an external tool ([example](https://tomeko.net/online_tools/hex_to_file.php?lang=en)) to make sure that it's correct.

## P2

Add a command line interface to P1 using the `std::env` functions. The application should accept the following format:
```
--in image.png --out out.txt
--in image.png
```
If `--out` is missing, the application will use a hardcoded default path.

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

## P3
Go back to P2 and replace the hand written command line parser with clap.