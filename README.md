# Notion Custom Domain

This application allows you to use a custom domain for your Notion page, replacing messy URLs with a cleaner, more memorable format. For example, you can access your page with your own custom domain instead of a messy URL like `https://example.notion.site/a8461811a3044446a2048fc054001b9d`.

## Features

- Custom Domain Mapping: Simplifies Notion page URLs by allowing you to set a custom domain.
- Configuration: Users can configure the API with a simple `.env` file.

## Prerequisites

- Rust
- Docker

### Additional packages

```bash
sudo apt install pkg-config libssl-dev
```

In order to run the application using `cargo run`, the `reqwest` library requires the `pkg-config` and `libssl-dev` packages to be installed

## Configuration

- `BIND_ADDRESS`: The IP address the application will use for hosting.
- `PORT`: The port number for hosting.
- `HOST`: The Notion origin URL.
- `NOTION_PAGE_ID`: The ID of the Notion page.
- `EXTERNAL_ADDRESS`: Actual external url for accessing application.
- `INJECT_TO_HEAD`: Custom code to inject right before `</head>`
- `INJECT_TO_BODY`: Custom code to inject right before `</body>`
