# Solana Programs Collection

A collection of Solana smart contracts demonstrating different programming patterns and use cases on the Solana blockchain.

## Programs

### 1. Counter Program
A native Solana program implementing a simple counter with multiple arithmetic operations.

**Features:**
- Initialize counter with value 1
- Double the current count
- Halve the current count
- Add a custom amount to the counter
- Subtract a custom amount from the counter
- Uses Borsh serialization for efficient data handling
- Implements overflow protection with saturating arithmetic

**Tech Stack:**
- Native Solana Program (no framework)
- Borsh for serialization
- TypeScript client with Bun runtime

**Checkout:** `counter/`

### 2. Escrow Program
An SPL token escrow program built with Anchor framework for secure token transfers between parties.

**Features:**
- Initialize escrow with specified token amount
- Lock tokens in a vault controlled by a PDA (Program Derived Address)
- Allow designated receiver to claim escrowed tokens
- Automatic vault creation using Associated Token Accounts
- Secure transfer validation with has_one constraint

**Tech Stack:**
- Anchor Framework v0.32.1
- SPL Token Program
- Associated Token Program

**Checkout:** `escrow/`

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) v1.14+
- [Anchor Framework](https://www.anchor-lang.com/docs/installation) v0.32.1 (for escrow program)
- [Bun](https://bun.sh/) (for counter client)
- Node.js 18+ (alternative to Bun)

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd solana-programs
```

2. Configure Solana CLI for your desired network:
```bash
# For devnet
solana config set --url devnet

# For localnet
solana config set --url localhost
```

## Building the Programs

### Counter Program
```bash
cd counter
cargo build-sbf
```

The compiled program will be in `target/deploy/contracts.so`

### Escrow Program
```bash
cd escrow
anchor build
```

The compiled program will be in `target/deploy/escrow.so`

## Deploying the Programs

### Counter Program
```bash
cd counter
solana program deploy target/deploy/contracts.so
```

### Escrow Program
```bash
cd escrow
anchor deploy
```

## Testing

### Counter Program
The counter program includes a TypeScript client for testing:

```bash
cd counter/src/client
bun install
bun test
```

### Escrow Program
```bash
cd escrow
anchor test
```

## Folder/file Structure

```
solana-programs/
├── counter/
│   ├── src/
│   │   ├── lib.rs              # Program entry point
│   │   ├── programs/
│   │   │   └── main.rs         # Counter logic implementation
│   │   └── client/             # TypeScript client
│   │       ├── index.ts        # Client interface
│   │       └── tests/          # Client tests
│   └── Cargo.toml
├── escrow/
│   ├── programs/
│   │   └── escrow/
│   │       ├── src/
│   │       │   └── lib.rs      # Escrow program logic
│   │       └── Cargo.toml
│   └── Cargo.toml              # Workspace config
└── vault/                      # In development
```

## Program Descriptions

### Counter Program Architecture
The counter program demonstrates native Solana program development without frameworks. It showcases:
- Custom instruction serialization using Borsh
- Account data management
- Signer validation
- State mutations

**Instructions:**
- `Init`: Initializes counter to 1
- `Double`: Multiplies counter by 2
- `Half`: Divides counter by 2
- `Add { amount }`: Adds specified amount
- `Subtract { amount }`: Subtracts specified amount

### Escrow Program Architecture
The escrow program uses the Anchor framework to simplify development and implements:
- PDA-based vault authority for secure token custody
- SPL Token transfers using Cross-Program Invocations (CPI)
- Account validation using Anchor constraints
- Automatic space calculation for account creation

**Instructions:**
- `initialize_escrow`: Creates escrow and deposits tokens into vault
- `claim_escrow`: Allows receiver to claim escrowed tokens

## Development

### Counter Program Development
The counter program is written in pure Rust without frameworks, providing insight into low-level Solana program development.

Key concepts demonstrated:
- Program entrypoint definition
- Account iteration and validation
- Borsh serialization/deserialization
- Error handling with ProgramError

### Escrow Program Development
Built with Anchor, showcasing modern Solana development practices:
- Declarative account validation
- Automatic space calculation
- Type-safe instruction handlers
- Built-in security checks

## Security Considerations

- **Counter**: Implements saturating arithmetic to prevent overflow/underflow
- **Escrow**: Uses PDA for vault authority to prevent unauthorized access
- **Escrow**: Validates receiver using `has_one` constraint
- All programs require proper signer validation

## Optimization

The workspace is configured with release optimizations:
- Link-Time Optimization (LTO) enabled
- Single codegen unit for maximum optimization
- Overflow checks enabled in release builds

## Resources

- [Solana Documentation](https://docs.solana.com/)
- [Anchor Framework](https://www.anchor-lang.com/)
- [SPL Token Program](https://spl.solana.com/token)
- [Solana Cookbook](https://solanacookbook.com/)

## PLAY WITH RUST, SOLANA

