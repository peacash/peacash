# Pea

## Install using [Cargo](https://doc.rust-lang.org/cargo/)

### Node

```bash
cargo install pea-node
```

### Wallet

```bash
cargo install pea-wallet
```

## Build

```bash
git clone https://github.com/peacash/pea
cd pea
rustup default nightly
cargo build
```

### Dependencies

#### Arch

```bash
sudo pacman -Sy
sudo pacman -S cmake clang
```

#### Debian

```bash
sudo apt update
sudo apt install cmake clang libssl-dev pkg-config
```

## Contribute

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.
