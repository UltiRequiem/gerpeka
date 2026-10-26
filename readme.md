# gerpeka 💀

> A better `pkill`, cross-platform.

**Status: proof of concept. Started. Never continued. Archived.**

## What it does

- **On Windows:** runs `taskkill /IM <name> /F`. Yes, that's it. That's the whole product.
- **On Linux and macOS:** runs `pkill <name>`. Which you already have. And which already works.

So the "better pkill" is, on the platform where `pkill` exists, `pkill` with extra steps,
and on the platform where it doesn't, a wrapper around the thing Windows already ships.

It prints `<name> was killed.` whether or not anything was killed. Optimistic. 🫡

## Usage

```sh
cargo install --git https://github.com/UltiRequiem/gerpeka
gerpeka firefox chrome slack
```

## Roadmap

- ~~Windows support~~
- ~~Everything else~~
- Be honest in the README ✅

## Why is this archived?

I proved the concept: you can, in fact, kill processes. The concept has been proven
and I have decided to stop while I'm ahead.

## License

MIT
