pub fn remove_notion_url(body: String) -> String {
    body.replace("https://www.notion.so", "http://localhost:3000")
        .replace("https://notion.so", "http://localhost:3000")
}

pub fn format_notion_page(body: String) -> String {
    remove_notion_url(body).replace(
        "</body>",
        r#"
    <script>
    const observer = new MutationObserver(function() {
      window.history.replaceState({}, "", "/");
    });
    observer.observe(document.getElementById("notion-app"), {
      childList: true,
      subtree: true,
    });
    </script>
    </body>
    "#,
    )
}
