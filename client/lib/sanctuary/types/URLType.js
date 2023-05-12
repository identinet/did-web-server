import $ from "sanctuary-def";

export const URLType = $.NullaryType("URL")(
  "https://github.com/identinet/identinet#URL",
)([])((x) => x instanceof URL);
