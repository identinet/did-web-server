import sanctuary from "sanctuary";
import dollar from "sanctuary-def";
import {
  ConcurrentFutureType,
  env as flutureEnv,
  FutureType,
} from "fluture-sanctuary-types";

import { PromiseType } from "./types/PromiseType.js";
import { URLType } from "./types/URLType.js";
import { ResponseType } from "./types/ResponseType.js";
import { NonZeroPortNumber, PortNumber } from "./types/PortNumber.js";

// TODO: add a simple function interface for adding / registering new types

const additionalTypes = [
  ResponseType,
  PromiseType,
  URLType,
  PortNumber,
  NonZeroPortNumber,
  ...flutureEnv,
];

const env = dollar.env.concat(additionalTypes);

const S = sanctuary.create({
  checkTypes: true,
  env,
});

const def = dollar.create({
  checkTypes: true,
  env,
});

S.def = def;

// // assign all types to S.types, see https://github.com/sanctuary-js/sanctuary/issues/717

// Export type constructors
const $ = { ...dollar };
S.map(([name, fn]) => $[name] = fn)([
  ["Future", FutureType],
  ["ConcurrentFuture", ConcurrentFutureType],
]);

export { $, def, S };
