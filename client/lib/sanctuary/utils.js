import S from "sanctuary";
/**
 * log prints an entry on the console and returns the passed in value.
 *
 * @param {String} msg - A message printed before the value.
 * @param {any} value - Any value.
 * @returns {any} Returns the passed in value.
 * @example
 * ```js
 * S.pipe([
 *   S.map(x => x.b = 2),
 *   log
 *   S.map(x => x.b = 3),
 * ])({a: 1})
 * ```
 */
export const log = (msg) => (value) => {
  console.log(`${msg}: ${S.show(value)}`);
  return value;
};
