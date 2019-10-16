import "../css/styles.css";
import "../web_components/code-block.ts";

(async () => {
  // Note: files in `crate/pkg/` will be created on the first build.
  await import("../crate/pkg/index");
})();
