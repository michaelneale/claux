# claux

Run claude instantly from your terminal anywhere any time. Similar to `@goose` from https://github.com/block/goose


<img width="740" height="419" alt="image" src="https://github.com/user-attachments/assets/6f130b3b-955a-4537-b273-94e42f829c80" />



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
