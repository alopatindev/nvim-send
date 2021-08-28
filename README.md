# nvim-send
Essentially [`nvr --nostart --remote-send <keys>`](https://github.com/mhinz/neovim-remote#demos) written in Rust.

## Usage
Run `nvim --listen /tmp/nvim.sock` (or for Windows users `nvim --listen \\.\pipe\nvim`)

```
$ cargo install nvim-send
$ nvim-send --remote-send '<esc>:echo "hello"<cr>' --servername /tmp/nvim.sock
```

## License
MIT/Apache-2.0
