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
- `ROUTE_PATHS`: The path to the Notion page (e.g. `/,/en`).
- `NOTION_PAGES`: The ID of the Notion page (e.g. `a8461811a3044446a2048fc054001b9d,aae83820e0124d50906dc50a3fefef20`).
- `EXTERNAL_ADDRESS`: Actual external url for accessing application.

### Inject tags

Place `head.html`, `body.html` files in the root directory. It should contain any HTML you wish to include at the end of the `<head>` or `<body>` section of the page.

```html
<style>
    @font-face {
        font-family: Pretendard-Regular;
        src: url("https://cdn.jsdelivr.net/gh/Project-Noonnu/noonfonts_2107@1.1/Pretendard-Regular.woff")
            format("woff");
        font-weight: 400;
        font-style: normal;
        font-display: swap;
    }
    * {
        font-family: Pretendard-Regular, ui-sans-serif, -apple-system,
            BlinkMacSystemFont, "Segoe UI", Helvetica, "Apple Color Emoji",
            Arial, sans-serif, "Segoe UI Emoji", "Segoe UI Symbol" !important;
    }
</style>
```

For example, if you create `head.html` like above, you can modify font of your page.
