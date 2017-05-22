
# xor-keysize-guess

This is a small command line application that can be used to guess the size of the key that was used to XOR encrypt a file.

## Installation

If you've not already done so, install rust: https://www.rust-lang.org/

Then install via cargo with:
```bash
$ cargo install xor
```


## Help

```bash
$ xor-keysize-guess --help
xor-keysize-guess 1.0.0
Gavyn Riebau
Guesses the most likely keysize used to XOR encrypt a given file.

USAGE:
	xor-keysize-guess [OPTIONS] --input <INPUT>

FLAGS:
	-h, --help       Prints help information
	-V, --version    Prints version information

OPTIONS:
	-i, --input <INPUT>            The input file for which the XOR encryption keysize will be guessed
	-k, --keysize <MAX_KEYSIZE>    The maximum key size to guess [default: 40]
```


