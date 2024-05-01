use std::{borrow::Cow, env::var};

use crate::AppState;

pub fn remove_notion_url(body: String, state: &AppState) -> String {
    body.replace("https://www.notion.so", state.external_address.as_str())
        .replace("https://notion.so", state.external_address.as_str())
}

pub fn format_notion_page(body: String, state: &AppState) -> String {
    let custom_body_string: Cow<String> = match var("INJECT_TO_BODY") {
        Ok(tags) => Cow::Owned(tags),
        Err(_) => Cow::Owned("".to_string()),
    };
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

      const topBar = document.querySelector(".notion-topbar") || document.querySelector(".notion-topbar-mobile");

      if (topBar && topBar.parentElement instanceof HTMLElement) {
        topBar.parentElement.remove();
      }

      const notionFrame = document.querySelector(".notion-frame");
      
      if (notionFrame) {
        notionFrame.style.removeProperty("height")
      }
    });
    observer.observe(document.getElementById("notion-app"), {
      childList: true,
      subtree: true,
    });
    </script>
    {{custom_body_string}}
    </body>
    "#;
    let inject_to_body = script
        .replace("{{external_address}}", state.external_address.as_str())
        .replace("{{origin_host}}", state.host.as_str())
        .replace("{{slug}}", state.notion_page_id.as_str())
        .replace("{{custom_body_string}}", custom_body_string.as_str());
    let custom_head_string: Cow<String> = match var("INJECT_TO_HEAD") {
        Ok(tags) => Cow::Owned(tags),
        Err(_) => Cow::Owned("".to_string()),
    };
    let inject_to_head = custom_head_string.to_string() + "</head>";

    remove_notion_url(body, &state)
        .replace("</body>", &inject_to_body)
        .replace("</head>", &inject_to_head)
}
