import $ from "sanctuary-def";

export const RequestType = $.NullaryType("Request")(
  "https://github.com/identinet/identinet#Request",
)([])((x) => {
  if (typeof Request !== "undefined") return x instanceof Request;
  else {
    // INFO: workaround to avoid node-fetch import - is not required in the browser
    return x?.constructor?.name === "Request";
  }
});
