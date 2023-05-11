// TODO: for server-side applications use the pine logger
import { getLogger as _getLogger, handlers, setup } from "std/log/mod.ts";

// Context is a Go-like immutable context that is passed to functions.
// New values are set with .withValue. A new conetxt is returned that includes the new value.
// If the value already exists, it's not modified.
// See for more details https://www.digitalocean.com/community/tutorials/how-to-use-contexts-in-go
// Usage:
// const ctx = new Context();
// ctx.withValue("key", "any value");
// ctx.value("key"));
export const Context = class {
  #values = {};
  constructor() {
  }
  // value retrieves a value from context
  // @param	key	who's value is retrieved
  // @returns	key's value
  value(key) {
    return this.#values[key];
  }
  // withValue sets a non-existing key to a value
  // @param	key	any value is allowed. Key is only set if it doesn't exist in context, yet
  // @param	value	any value is allow
  // @returns	new context that includes key with value
  withValue(key, value) {
    const ctx = new Context();
    ctx.#values = { [key]: value, ...this.#values };
    return ctx;
  }
};

// getLogger takes a Context and returns the logger from the context or an empty logger
// Optional name that is used if
// @param	ctx	Context object
// @param	name	optional name that is used if Context doesn't include a logger
// @param	level	optional log level that is used if Context doesn't include a logger
// @returns	logger instance
export function getLogger(ctx, name, level) {
  let logger = ctx?.value("logger");
  if (!logger) {
    const _level = level ? level : "INFO";
    setup({
      handlers: {
        console: new handlers.ConsoleHandler(_level),
      },
      loggers: {
        [name]: {
          level: _level,
          handlers: ["console"],
        },
      },
    });
    logger = _getLogger(name);
  }
  return logger;
}
