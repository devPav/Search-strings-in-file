This cli application provide search strings in txt files.

cargo run %1 %2 [%3]

%1 - a query string

%2 - a txt file path

%3 - an optional argument that allows you to search with case sensitive when it's "1", else insensitive case.
     If the argument doesn't use, search with case insensitive will be when you have "CASE_INSENSITIVE" env var

Result: stdout with string lines from txt file in which the search quary occurs

Example:
cargo run to poem.txt 1 > output.txt