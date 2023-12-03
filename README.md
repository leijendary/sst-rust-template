# SST Rust template

- This template is intended for personal use
- Sample classes are included

# Technologies used:

- Rust
- AWS
- [SST](https://sst.dev)

### To run the template:

1. Copy `.env` to `.env.local`.
2. Add your local machine specific configuaration to `.env.local`.
3. Add secrets via SST. Refer to https://docs.sst.dev/config.
4. (Optional) use `schema.sql` as your base database schema.
5. Run `npm run dev` using your terminal or use VSCode's **Run and Debug** tab.

For more information, go to https://sst.dev

# Deploying:

## Pre-requisites:

Run the following commands before deploying:

1. `brew tap cargo-lambda/cargo-lambda`
2. `brew install cargo-lambda`
3. `brew install zig`

## Deploy to stage:

`npm run deploy -- --stage <stage>`

Where `<stage>` is the name of the environment. Example: `prod`.
