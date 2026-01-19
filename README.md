# claux

Run claude instantly from your terminal anywhere any time. Similar to `@goose`. 


## Build

```bash
cargo build --release
```

## Install to PATH

```bash
cargo install --path .
```

This installs the binary to `~/.cargo/bin/claux` (make sure `~/.cargo/bin` is in your PATH).

## Alias as `@claude`

Add this to your shell config (`~/.zshrc` or `~/.bashrc`):

```bash
alias @claude="claux"
```

Then reload your shell:

```bash
source ~/.zshrc  # or source ~/.bashrc
```

Now you can use it as:

```bash
@claude "your prompt here"
```

## Usage

```bash
claux your prompt here
```

This is equivalent to running:
```bash
claude --dangerously-skip-permissions -p "your prompt here"
```
