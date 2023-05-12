import $ from "sanctuary-def";

export const ResponseType = $.NullaryType("Response")(
  "https://github.com/identinet/identinet#Response",
)([])((x) => {
  if (typeof Response !== "undefined") return x instanceof Response;
  else {
    // INFO: workaround to avoid node-fetch import is not required in the browser
    return x?.constructor?.name === "Response";
  }
});
