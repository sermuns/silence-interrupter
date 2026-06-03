<div align="center">

# `silence-interrupter`

<img src="./media/illustrative-image.png" width=700>

</div>

For when you're too deeply locked in, and just need that adrenaline-kick every so often.

## Features

1. Will play a random "brainrot"-sound every so often, according to the random time interval you specify.

## Usage

Install

```sh
cargo install --git https://github.com/sermuns/silence-interrupter
```

and run it, here's the usage:

```present cargo run -- -h
Usage: silence-interrupter [OPTIONS] --range <start>..<end>

Options:
  -r, --range <start>..<end>  Possible random time range
  -g, --gain <GAIN>           [default: 1]
  -h, --help                  Print help
  -V, --version               Print version
```

### Make it always run in the background

A good(bad) idea is to create a `systemd` service so it auto-starts with your login session:

1. Create `~/.config/systemd/user/silence-interrupter.service` with the contents

   ```systemd
   [Unit]
   Description=silence-interrupter

   [Service]
   Type=simple
   ExecStart=%h/.cargo/bin/silence-interrupter --range 1m..10m # or whatever range you want
   Restart=on-failure

   [Install]
   WantedBy=default.target
   ```

2. Enable and start the service:

   ```sh
   systemctl --user enable --now silence-interrupter
   ```

3. Welcome to your new life!

## Contributing

We need more sounds. Please contribute!

## Disclaimer

This project is 100% certified human-slop. **No artificial intelligence was used in the making of this.**

<a href="https://brainmade.org/">
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://brainmade.org/white-logo.svg">
  <source media="(prefers-color-scheme: light)" srcset="https://brainmade.org/black-logo.svg">
  <img alt="brainmade" src="https://brainmade.org/white-logo.svg">
</picture>
</a>
