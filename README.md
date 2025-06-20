
# MD2HTML

Simple CLI program to convert markdown files to HTML buit in rust.



## Features

- Reads Markdown from a file
- Converts to HTML using pulldown-cmark
- Saves output to a new file
- Command line interface with argument parsing


## Installation

```bash
  git clone https://github.com/M4rcinWisniewski/md2html.git
    cd md2html
    cargo build --release
```
    
## Usage/Examples

```
cargo run -- --input ./example.md
```
or if you have a built binary:
```
./md2html --input ./example.md
```
### Remember to put your file into include directory!


## Contributing

Contributions are always welcome!



## Authors

- [@M4rcinWisniewski](https://github.com/M4rcinWisniewski)

