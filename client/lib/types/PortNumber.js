import $ from "sanctuary-def";

/**
 * PortNumber between 0 and 65535.
 */
export const PortNumber = $.NullaryType("PortNumber")(
  "https://github.com/identinet/identinet/types#PortNumber",
)([$.PositiveNumber])((x) => x <= 65535);

/**
 * NonZeroPortNumber between 1 and 65535.
 */
export const NonZeroPortNumber = $.NullaryType("NonZeroPortNumber")(
  "https://github.com/identinet/identinet/types#NonZeroPortNumber",
)([PortNumber])((x) => x !== 0);
