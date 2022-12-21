# Recap

Stripped down [Read](https://doc.rust-lang.org/std/io/trait.Read.html) trait:
```rs
trait Read {
    // Reads some bytes into the buffer and returns how much it read.
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize>;

    // Reads the whole buffer. If it doesn't have enough bytes, it errors out.
    fn read_exact(&mut self, buf: &mut [u8]) -> std::io::Result<()>;

}
```
Take a `Read` as generic:
```rs
use std::io::Read;

fn f<R: Read>(reader: &mut R) -> std::io::Result<()> {}
```

# Problems

## P1

Read the following binary format that represents an archive format and write the files back to the disk.

Primitive types:

| name   | description                      |
| ------ | -------------------------------- |
| u32    |  unsigned number with 32 bits; use [from_ne_bytes](https://doc.rust-lang.org/std/primitive.u32.html#method.from_ne_bytes) to convert bytes to it |
| buffer | an u32 prefix for the size, followed by size bytes |
| string | an u32 prefix for the size, followed by size bytes with UTF8 bytes |


The format root has the following fields:
| name        | type           | description                      |
| ----------- | -------------- | -------------------------------- |
| magic       | u32            | the "FIIA" string                |
| description | string         | short description of the archive |
| count       | u32            | how many files are following     |
| files       | archive_file[] | `count` x files                  |

`archive_file` format:
| name      | type   | description          |
| --------- | ------ | -------------------- |
| file_name | string | the file name        |
| data      | buffer | the data of the file |

Implement the structures and functions needed to read the format in a separate module file named `archive.rs`. The functions that do the reading must take a generic type that implements `Read` as argument, as shown above.

In the main file, deserialize an archive and write each file to the disk, and print the description to stdout.
Example archives at [archives](archives/).

## P2

Make an application that takes a folder as input, and creates this archive format with the files in it. Use the `Write` trait for the writing functions.