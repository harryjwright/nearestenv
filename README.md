# NearestEnv

A lightweight Rust utility that recursively searches upward for `.env` files and strictly manages required environment variables.

## Purpose

`NearestEnv` solves configuration issues in multi-root workspaces and deep directory structures by:

- Traversing parent directories to locate and load the nearest `.env` file.
- Preserving existing environment variables (such as those supplied by Docker Compose) rather than overwriting them.
- Enforcing a strict "no var, no start" policy to ensure applications fail fast if required configuration is missing.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
nearest_env = { git = "https://github.com/harryjwright/nearestenv.git", branch = "main" }
```

In your application entry point:

```Rust
use nearest_env::NearestEnv;

fn main() {
    // Search upward and load the nearest .env file
    NearestEnv::load();

    // Fetch a required variable (terminates startup if missing)
    let database_url = NearestEnv::get_env("DATABASE_URL");
}
```

# License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
