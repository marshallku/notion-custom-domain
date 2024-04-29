pub fn remove_notion_url(body: String) -> String {
    body.replace("https://www.notion.so", "http://localhost:3000")
        .replace("https://notion.so", "http://localhost:3000")
}
