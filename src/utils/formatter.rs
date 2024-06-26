use crate::{utils::file::read_file, AppState};
use serde_json::to_string;

pub fn modify_notion_url(body: String, state: &AppState) -> String {
    body.replace("https://www.notion.so", state.external_address.as_str())
        .replace("https://notion.so", state.external_address.as_str())
        .replace("/_assets/", "https://www.notion.so/_assets/")
}

pub fn format_notion_page(body: String, state: &AppState) -> String {
    let custom_body_string = read_file("body.html").unwrap_or("".to_string());
    let script = r#"
    <script>
    window.EXTERNAL_ADDRESS="{{external_address}}";
    window.ORIGIN_HOST="{{origin_host}}";
    window.SLUGS={{slugs}};
    window.CONFIG.domainBaseUrl = window.ORIGIN_HOST;
    const replaceState = window.history.replaceState;
    window.history.replaceState = function(state, title, url) {
      if (Object.values(window.SLUGS).find(x => url.includes(x))) {
        return;
      }

      return replaceState.apply(window.history, arguments);
    };
    const observer = new MutationObserver(function() {
      if (!Object.keys(window.SLUGS).find(x => window.location.pathname === x)) {
        const selected = Object.entries(window.SLUGS).find(([key, value]) => value === window.location.pathname.slice(1));
        window.history.replaceState({}, "", (selected ? selected[0] : "") || "/");
      }

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
        .replace(
            "{{slugs}}",
            &to_string(&state.path_to_notion_map).unwrap_or("{}".to_string()),
        )
        .replace("{{custom_body_string}}", custom_body_string.as_str());
    let custom_head_string = read_file("head.html").unwrap_or("".to_string());
    let inject_to_head = custom_head_string.to_string() + "</head>";

    modify_notion_url(body, &state)
        .replace("</body>", &inject_to_body)
        .replace("</head>", &inject_to_head)
}
