
[![CI](https://github.com/skewnart/rgrep/actions/workflows/ci.yml/badge.svg)](https://github.com/skewnart/rgrep/actions/workflows/ci.yml)
[![dependency status](https://deps.rs/repo/github/skewnart/rgrep/status.svg)](https://deps.rs/repo/github/skewnart/rgrep)

# rgrep

## Download & Build

``` bash
git clone https://github.com/skewnart/rgrep && cd rgrep
cargo b -r 
```

Take the executable and keep it in a PATH

## Usage

``` bash
> rgrep searchstr file.txt

The word is searchstr.
```
or
``` bash
> cat file.txt | rgrep searchstr

The word is searchstr.
```

Case insensitive usage :
``` bash
> rgrep searchstr file.txt -i

The word is searchstr.
Searchstr is a good word.
```