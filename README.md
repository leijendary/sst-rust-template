# SST Rust template

- This template is intended for personal use
- Sample classes are included

## Technologies used:

- Rust
- AWS
- [SST](https://sst.dev)

## Run locally

1. Copy `.env.example` to `.env`.
2. Add your local machine specific configuaration to `.env`.
3. Add secrets via SST. Refer to https://docs.sst.dev/config.
4. (Optional) use `migrations/1_init.sql` as your base database schema. I usually use [Neon](https://neon.tech) for branching.
5. Run `npm run dev` using your terminal or use VSCode's **Run and Debug** tab.

For more information, go to https://sst.dev

## Deploying

### Pre-requisites

Run the following commands before deploying:

1. `brew tap cargo-lambda/cargo-lambda`
2. `brew install cargo-lambda`
3. `brew install zig`

### IAM Role

To deploy the stack to an AWS account, you will need to follow these [IAM Permissions](https://docs.sst.dev/advanced/iam-credentials#iam-permissions).

### Deploy to a stage

`npm run deploy -- --stage <stage>`

Where `<stage>` is the name of the environment. Example: `prod`.
