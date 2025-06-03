# Solana Sample Program

A Solana program template built using the Anchor framework, providing a foundation for building Solana applications.

## Prerequisites

- Rust (latest stable version)
- Solana CLI tools
- Anchor Framework (version 0.28.0)
- Node.js and Yarn (for package management)

## Project Setup

1. Install Solana CLI tools:

```bash
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

2. Install Anchor Framework:

```bash
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install 0.28.0
avm use 0.28.0
```

3. Clone the repository and install dependencies:

```bash
git clone <repository-url>
cd sample
yarn install
```

4. Build the program:

```bash
anchor build
```

## Project Structure

```
sample/
├── app/                  # Frontend application directory
├── programs/            # Solana program directory
│   └── sample/
│       ├── src/
│       │   ├── instructions/  # Program instructions
│       │   ├── state/        # Program state definitions
│       │   ├── constants.rs  # Program constants
│       │   ├── error.rs      # Custom error definitions
│       │   └── lib.rs        # Program entry point
├── tests/               # Test directory
├── migrations/          # Database migrations
├── Anchor.toml          # Anchor configuration
├── Cargo.toml          # Rust dependencies
├── package.json        # Node.js dependencies
└── tsconfig.json       # TypeScript configuration
```

## Development

### Local Development

1. Start a local Solana validator:

```bash
solana-test-validator
```

2. Build and deploy the program:

```bash
anchor build
anchor deploy
```

### Running Tests

The project includes both TypeScript and Rust tests.

To run all tests:

```bash
anchor test --skip-local-validator
```


```bash
anchor test
```

Or run specific test suites:

```bash
# Run Rust tests
cargo test

# Run TypeScript tests
yarn test
```

## Configuration

### Solana Configuration

Make sure your Solana CLI is configured for local development:

```bash
solana config set --url localhost
```

### Wallet Setup

The project uses the default Solana wallet location:

```
~/.config/solana/id.json
```

To create a new wallet:

```bash
solana-keygen new
```

## Dependencies

### Rust Dependencies

- anchor-lang = "0.28.0"

### Node.js Dependencies

- @project-serum/anchor
- @solana/web3.js
- TypeScript
- Mocha/Chai for testing

## Scripts

Available npm/yarn scripts:

- `anchor test`: Run all tests
- `anchor build`: Build the program
- `anchor deploy`: Deploy to local network

## License

This project is open source and available under the MIT License.

## Support

For support, please open an issue in the repository or contact the maintainers.
