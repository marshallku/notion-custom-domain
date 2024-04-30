use crate::AppState;

pub fn remove_notion_url(body: String, state: &AppState) -> String {
    body.replace("https://www.notion.so", state.external_address.as_str())
        .replace("https://notion.so", state.external_address.as_str())
}

pub fn format_notion_page(body: String, state: &AppState) -> String {
    remove_notion_url(body, &state).replace(
        "</body>",
        r#"
    <script>
    const observer = new MutationObserver(function() {
      window.history.replaceState({}, "", "/");

      const topBar = document.querySelector(".notion-topbar");

      if (topBar && topBar.parentElement instanceof HTMLElement) {
        topBar.parentElement.remove();
      }
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
