# Recap

Characters are stored in a string using the UTF8 encoding. That means that a single character (codepoint) can take several bytes. For instance, the character `ă` is written as 2 bytes: `[196, 131]`. The character 🎶 is written as 4 bytes: `[240, 159, 142, 182]`.

- [str docs](https://doc.rust-lang.org/std/primitive.str.html)
- [String docs](https://doc.rust-lang.org/std/string/struct.String.html)

To iterate over the characters of a string, use `.chars()`:
```rs
let s = "abc";
for i in s.chars() {
    println!("{}", i);
}
```

# Reading and writing text files example

```rs
use std::{io, fs};

fn do_stuff() -> Result<(), io::Error> {
    let s = fs::read_to_string("src/main.rs")?;
    fs::write("copy.rs", &s)?;

    Ok(())
}
```

# Problems
For each problem, make sure to test several success and failure cases.

## P1
Read a text file. Calculate and print its longest line considering the number of bytes, and the longest line considering the number of actual characters.

Example input:
```
strings are fun
🎁🎶🎉👀🎈🎃🍕☕🍉
rust
supercalifragilisticexpialidocious
```

## P2
Implement the [ROT13](https://en.wikipedia.org/wiki/ROT13) cipher. This cipher will rotate each ASCII letter by 13. Be mindful that lowercase letter transform into lowercase letters, and uppercase letters into uppercase letters.

If any non-ASCII character is encountered, print an error message and stop. Read and write the text however you want.

## P3
Have a list of abbreviations (this can be hardcoded):
```
pentru pt
pentru ptr
domnul dl
doamna dna
```
Read a phrase from a file and replace any word that is an abbreviation into its correct form. Assume that the phrase only contain spaces for delimitations.

Example:

`Am fost la dl Matei pt că m-a invitat cu o zi înainte`

=>

`Am fost la domnul Matei pentru că m-a invitat cu o zi înainte`

## P4
Read the [hosts](https://en.wikipedia.org/wiki/Hosts_(file)) file from your system (Windows: `C:\Windows\System32\drivers\etc\hosts`, Unix: `/etc/hosts`). Split the lines, ignore the lines that start with `#`, and print the content in the format below. Take into considerations only the first two text columns of each entry.
```
localhost => 127.0.0.1
...
```
Note: the file might be fully commented if you're running a security product. Try with a file written in another location if that is the case.

