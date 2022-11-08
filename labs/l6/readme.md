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

[ureq](https://crates.io/crates/ureq) is a crate for making HTTP requests that aims to be simple to use.

Example usage:
```rs
fn main() -> Result<(), ureq::Error> {
    let body: String = ureq::get("http://example.com")
        .call()?
        .into_string()?;
    Ok(())
}
```
This code sends a `GET` request to `example.com` and effectively downloads the page, converting it into a string. For this example, the string is just HTML markup code, but you'll usually get JSON/XML strings when calling web APIs.

## P2

Download the information of all emojis from [emojihub](https://emojihub.herokuapp.com/api/all). The JSON has the following format:
```json
[
   // ...
    {
        "name": "slice of pizza ≊ pizza",
        "category": "food and drink",
        "group": "food prepared",
        "htmlCode": [
            "&#127829;"
        ],
        "unicode": [
            "U+1F355"
        ]
    },
    // ...
]
```

Use `serde_json` as described in the [previous lab](../l5/readme.md#bonus) to parse this JSON. Group emojis by their `group` (use a map), and print to a file every group and either the name of the emojis, or the actual unicode emoji.