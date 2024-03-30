## scraper
Continuously captures copied text, complete the template, and then copy it again.

## Documentation
### Dependencies
This program is built with rust.

[https://www.rust-lang.org/learn/get-started](https://www.rust-lang.org/learn/get-started)
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```


### How to build
Clone this repository and run this command.

```
cargo build
```

Now the program is placed in `./target/debug` directory.

### How to run
1. command line template
   
```
./copier "[[first]] -> [[second]] -> [[third]]"

or

cargo run "[[first]] -> [[second]] -> [[third]]"
```

2. template file

template.txt
```
[[title]]

[[author]]

[[summary]]
```

```
./copier --file ./template.txt

or

cargo run -- --file ./template.txt
```

### How to use
After startup,
```
[capturing first ...]
>>
```

the program is waiting for text copy or input.

Just copy any text continuosly.
```
[capturing first ...]
>>
Google -> [[second]] -> [[third]]
[capturing second ...]
>>
Google -> Wikipedia -> [[third]]
[capturing third ...]
>>
Google -> Wikipedia -> encyclopedia
[capture finished, send to clipboard]
[capturing first ...]
>>
```

Or you can type words.
You can end input by sending signal `Ctrl+D`(eof) twice.
```
[capturing first ...]
>> hello^D
hello -> [[second]] -> [[third]]
[capturing second ...]
>> my name^D
hello -> my name -> [[third]]
[capturing third ...]
>> is kim^D
hello -> my name -> is kim
[capture finished, send to clipboard]
[capturing first ...]
>>
```

After capturing process has finished, complete template is copied to the clipboard, and the process restarts.

Exit program by sending signal `Ctrl+C`.


