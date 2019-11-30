# prf (Pull Request Fetch)

I was troubled by `fpr` or `prf` but I decided to use 'prf' for ease of typing.

## Overview

[![Actions Status](https://github.com/k-nasa/prf/workflows/CI/badge.svg)](https://github.com/k-nasa/prf/actions)
[![crate-name at crates.io](https://img.shields.io/crates/v/prf.svg)](https://crates.io/crates/prf)

A tool for easily fetching submitted PR into a local repository

For example, I tried to get a PR branch locally by typing the following command a while ago.
`git fetch upstream pull / 503 / head: random_merge`

However, two pieces of information are needed to execute this command.
One is the PR number and the second is the branch name.

It's a little cumbersome to check and enter this. So I made a tool that can get PR branch locally only with PR number.

## Example

Use it like this. Just enter the PR number.

`prf 55`

Also, by default it gets from origin, but you may want to change the remote. In that case, it is like this.

`prf 55 upstream`

## Installation

### Pre-compiled executables

Get them [here](https://github.com/k-nasa/prf/releases)

### using homebrew

```
brew install k-nasa/tap/prf
```

### using cargo

```
cargo install prf
```

##### Installation of cargo itself.

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Usage

```
prf 0.1.0
k-nasa <htilcs1115@gmail.com>
Tool to fetch PR branch to local repository

USAGE:
    prf [OPTIONS] <pr_no> [remote]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --owner <owner>              repository owner (By default it uses the local repository's remote url)
    -r, --repository <repository>    repository name (By default it uses the local repository's remote url)

ARGS:
    <pr_no>     pull request number fetching to local
    <remote>     [default: origin]
```

## Contribution

1. Fork it ( http://github.com/k-nasa/prf)
2. Create your feature branch (git checkout -b my-new-feature)
3. Commit your changes (git commit -am 'Add some feature')
4. Push to the branch (git push origin my-new-feature)
5. Create new Pull Request

## Licence

[MIT](https://github.com/k-nasa/prf/blob/master/LICENCE)

## Author

[k-nasa](https://github.com/k-nasa)

[my website](https://k-nasa.me)
