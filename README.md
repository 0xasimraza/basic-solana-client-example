# Basic Solana Counter Program Example

A simple Solana program that implements a counter with increment and decrement functionality, along with a Rust client example.

## Project Structure

```
counter-rs-test-example/
├── programs/
│   └── counter-rs-test-example/    # Solana program
│       ├── src/
│       │   ├── instructions/       # Program instructions
│       │   ├── state/             # Program state definitions
│       │   └── lib.rs             # Program entry point
├── tests/
│   ├── client/                     # Rust client implementation
│   │   └── src/
│   │       └── main.rs            # Client code
│   └── counter-rs-test-example.ts  # TypeScript tests
└── Anchor.toml                     # Anchor configuration
```

## Quick Start

1. Build the program:

```bash
anchor build
```

2. Start local validator:

```bash
solana-test-validator --reset
```

3. Deploy the program:

```bash
anchor deploy
```

4. Run the Rust client:

```bash
cargo run -p rs
```
