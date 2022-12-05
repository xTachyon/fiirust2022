# Homework

Every student must choose one of the following problems as homework, or invent one and ask the course professor or the lab associate to approve it.

Every homework must be able to receive command line arguments that changes its behavior, including a help command that prints the available commands. You may use [clap](https://crates.io/crates/clap) for this or a hand rolled parser.

The applications must compile without warnings, and must propagate errors to main correctly.

# Ascii table gen

Read CSV and JSON files (figure out which is which by its extension) and generate an ASCII table that will be outputed to stdout or a file. For CSV files, make sure to support entries that include the separator.

Output examples: [1](table1.txt), [2](table2.txt).

Command line options:
- the input file
- the output file or stdout (default: stdout)
- if there's a separating line between content lines, like in the [second example](table2.txt) (default: no)
- how to align the cells (left, center, right) (default: left)


# Print Reddit posts

Download the news feed from a subreddit from Reddit ([ex](https://www.reddit.com/r/foxes/hot.json)) and print the following information for each post: creation date, title, and link to post.

Command line options:
- the name of the subreddit
- the sort order: hot, new, top (default: hot)

Bonus: print only the new posts every N seconds (keep a list of posts that have already been printed).

Useful crates: [ureq](https://crates.io/crates/ureq) for downloading over http, [serde_json](https://crates.io/crates/serde_json) for json, [chrono](https://crates.io/crates/chrono).


# Recursive grep

Create a tool that searches for a substring in every file in a directory and every subdirectory, and print the name of the file and the relevant line with its line number.

Command line options:
- string to search for
- max number of lines (after which the application exits) (default: infinite)
- ignore case (default: no)
- only count (print only the number of matches per file, without the lines) (default: no)

Bonus: have an option to allow for [regex](https://crates.io/crates/regex) searching.