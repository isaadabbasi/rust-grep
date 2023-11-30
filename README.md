# Rust-grep
grep like basic cli-app built on rust 🦀

### How to usage
It' needs years of practice, but I will tell you a shorter way to do it.

The app takes 3 input parameters
- **"query":** text you want to search
- **"filename":** the name of file you want to search in
- **"i" or "-i":** if you want search to be case insensitive

As Example: 
- **poem.txt** is the file
- **query:** I sleep
- **"i" or "-i":**

#### running through codebase: 
```
cargo build;
cargo run "I sleep" poem.txt
// OR 
cargo run "i sleep" poem.txt
// both should print last two lines of poem.txt file
```