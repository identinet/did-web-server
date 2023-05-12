import $ from "sanctuary-def";
import S from "sanctuary";

export const PromiseType = $.NullaryType("Promise")(
  "https://github.com/identinet/identinet#Promise",
)([])((x) => S.type(x).name === "Promise");
