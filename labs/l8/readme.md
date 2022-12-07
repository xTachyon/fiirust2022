# Recap

Defining and implementing a trait:
```rs
trait MyTrait {
    fn print_static();
    fn print_member(&self);
    fn set(&mut self, x: u32);
    fn consume(self);
}

struct S {
    x: u32,
}
impl MyTrait for S {
    fn print_static() {
        println!("wow! 🎈");
    }
    fn print_member(&self) {
        println!("{}", self.x);
    }
    fn set(&mut self, x: u32) {
        self.x = x;
    }
    fn consume(self) {
        println!("and we're done");
    }
}

fn main() {
    S::print_static();
    // static fn

    S { x: 5 }.print_member();
    // const self ref

    let mut x = S { x: 10 };
    x.set(345);
    x.print_member();
    // mutable self ref

    x.consume();
    // self

    // x.print_member();
    // variable was moved
}
```

# Problems

## P1

For this problem you are supposed to write an application that emulates a terminal that implements several commands. To emulate a command, make a trait with the following functions:
- get_name: returns a string with the name of the command
- exec: takes a slice of strings with the command arguments

Make a `struct` for each command and implement the above trait for the following commands:
- ping: prints "pong!"
- times: prints how many times this command (only this command!) has been called
- cp: copies the file mentioned in the first argument into the path mentioned in the second argument, similar to Unix's `cp` command

Make a `struct` that keeps the commands in a collection, reads the commands from stdin, and executes them. This struct needs to have the following functions:
- new: creates a new instance
- register: adds the new command to the collection
- run: reads lines from stdin, splits them based on spaces, uses the first word as the command and searches it in the collection, and executes the command with the remaining arguments. This function should treat the case where the command is not valid or if the line is empty. It will continue to run even on error cases.

Example `main`:
```rs
fn main() {
    let mut terminal = Terminal::new();

    terminal.register(Box::new(PingCommand {}));
    terminal.register(Box::new(TimesCommand { count: 0 }));
    terminal.register(Box::new(CpCommand {}));

    terminal.run();
}
```

Bonus: add a special `stop` command that stops the execution, and suggest the correct command if the uses misspells it (eg: if it says `TIMES`).

## Bonus
[SQLite](https://www.sqlite.org/lang.html) is the most used database engine on the planet, being used in desktop, laptops, phones, aircrafts, TVs, browsers, antiviruses, etc. The difference between SQLite and other databases is that SQLite is designed to not have the need of a server, and is routinely used by applications to store data (eg: your phone stores the contact list in a SQLite database).

[rusqlite](https://crates.io/crates/rusqlite) is the crate that is used the most to interact with SQLite databases. Add the following under `dependencies`:
```toml
rusqlite = { version = "0.28.0", features = ["bundled"] }
```

Opening and creating tables if they not exists (because we'll probably run this multiple times):
```rs
let conn = Connection::open("persons.db")?;
let create = r"
create table if not exists persons (
    name text    not null,
    age  integer not null
);
";
conn.execute(create, ())?;
```
Note: you need `execute_batch` if you want to create more than a table.

Inserting data in a table:
```rs
let name = "cătălin";
let age = 22;
conn.execute("insert into persons (name, age) values (?1, ?2);", (name, age))?;
```
Notice that the SQL query inserts the values `?1` and `?2`. This gets replaced by the arguments that follow.

Selecting data from the database:
```rs
struct Person {
    name: String,
    age: u8
}

let mut stmt = conn.prepare("select * from persons")?;
let person_iter = stmt.query_map([], |row| {
    Ok(Person {
        name: row.get("name")?,
        age: row.get("age")?
    })
})?;
for i in person_iter {
    let i = i?;
    println!("name={}, age={}", i.name, i.age);
}
```
The `query_map` function accepts as first argument the possible added arguments with `?1`, `?2`, etc., just like `execute` does. In this case, this happens to be empty.

## P2
Add a bookmark "bm" command that uses a SQLite database to store bookmarks. Each bookmark can have a name and an url.

Based on the first argument, the command will do the following:
- `add <name> <url>`: adds an url to the database
- `search <name>`: prints all the names and the urls that contains `name` in their name

Usage example:
```
bm add foxes https://www.reddit.com/r/foxes/
bm add rust_book https://doc.rust-lang.org/book/
bm add foxes_pics https://www.boredpanda.com/beautiful-fox-pictures/

bm search fox
bm search rust
```