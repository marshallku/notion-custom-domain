use crate::AppState;

pub fn remove_notion_url(body: String, state: &AppState) -> String {
    body.replace("https://www.notion.so", state.external_address.as_str())
        .replace("https://notion.so", state.external_address.as_str())
}

pub fn format_notion_page(body: String, state: &AppState) -> String {
    let script = r#"
    <script>
    window.EXTERNAL_ADDRESS="{{external_address}}";
    window.ORIGIN_HOST="{{origin_host}}";
    window.SLUG="{{slug}}";
    window.CONFIG.domainBaseUrl = window.ORIGIN_HOST;
    const replaceState = window.history.replaceState;
    window.history.replaceState = function(state, title, url) {
      if (url.includes(window.SLUG)) {
        return;
      }

      return replaceState.apply(window.history, arguments);
    };
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
    "#;
    let formatted = script
        .replace("{{external_address}}", state.external_address.as_str())
        .replace("{{origin_host}}", state.host.as_str())
        .replace("{{slug}}", state.notion_page_id.as_str());

    remove_notion_url(body, &state).replace("</body>", &formatted)
}
