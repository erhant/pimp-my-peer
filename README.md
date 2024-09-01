# Pimp my Peer

Tired of random peer ids in your favorite peer-to-peer network with libp2p? Well, fret not, here you can mine your favorite good-looking peer ids at ease.

## Installation

```sh
cargo install --git https://github.com/erhant/pimp-my-peer
```

## Usage

See usage with `-h` or `--help` options:

```sh
pimp-my-peer --help
```

### Keywords

You can provide a target substring, prefix or suffix.

### Methods

Then, you choose the desired methodology for the search, from the following:

- `random`: tries a random key for each iteration
- `linear`: starts incrementing the given seed
- `identity`: no search, simply generates the peer id from the given seed

### Examples

- A randomly generated result that ends with `bye`:

```sh
pimp-my-peer -m=random -e=bye
```

- The corresponding peer id of a given seed:

```sh
pimp-my-peer -m=identity --seed=cafecafecafecafecafecafecafecafecafecafecafecafecafecafecafecafe
```
