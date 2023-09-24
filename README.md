# Minigrep

I like to learn new languages by re-implementation of tools and commands that I use on a daily basis.

`grep` is a useful command tool for finding substrings within a string in Unix-like operating systems.

With `minigrep` we will read a file and return lines which contain specific string (passed as arguments) in that file.

The functionnalities will be developed in TDD.

## Run
 > cargo run -- "j'ai vu" Rimbaud.txt 

Or

 > IGNORE_CASE=1 cargo run -- "j'ai vu" Rimbaud.txt

## TODO

- [x] Read a file
- [x] Search for specific lines passed as argument
- [x] Sensitive and insensitive query
- [x] Allow user to configure sensitivity via environment variable
- [x] Write errors to `stderr
