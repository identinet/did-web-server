/* DIDDocument is a builder class for DID Documents.
 */
export class DIDDocument {
  #builder = [];
  constructor() {
    this.withContext("https://www.w3.org/ns/did/v1");
  }
  #setProperty(property, value) {
    return (object) => {
      if (typeof value !== "undefined") {
        object[property] = value;
      }
      return object;
    };
  }
  #appendProperty(property, value) {
    return (object) => {
      if (typeof value !== "undefined") {
        if (!object[property]) {
          object[property] = [];
        }
        object[property].push(value);
      }
      return object;
    };
  }

  withContext(context) {
    this.#builder.push(this.#appendProperty("context", context));
    return this;
  }
  withSubject(subject) {
    this.#builder.push(this.#setProperty("id", subject));
    return this;
  }
  withController(controller) {
    this.#builder.push(this.#setProperty("controller", controller));
    return this;
  }
  withAuthentication(authentication) {
    this.#builder.push(this.#appendProperty("authentication", authentication));
    return this;
  }
  withAssertionMethod(assertionMethod) {
    this.#builder.push(
      this.#appendProperty("assertionMethod", assertionMethod),
    );
    return this;
  }
  withKeyAgreement(keyAgreement) {
    this.#builder.push(
      this.#appendProperty("keyAgreement", keyAgreement),
    );
    return this;
  }
  withCapabilityInvocation(capabilityInvocation) {
    this.#builder.push(
      this.#appendProperty("capabilityInvocation", capabilityInvocation),
    );
    return this;
  }
  withCapabilityDelegation(capabilityDelegation) {
    this.#builder.push(
      this.#appendProperty("capabilityDelegation", capabilityDelegation),
    );
    return this;
  }
  withVerificationMethod(
    { id, type, controller, publicKeyJwk, publicKeyMultibase },
  ) {
    const verificationMethod = [
      this.#setProperty("controller", controller),
      this.#setProperty("publicKeyJwk", publicKeyJwk),
      (object) => {
        if (typeof publicKeyJwk !== "undefined") {
          return object;
        }
        return this.#setProperty("publicKeyMultibase", publicKeyMultibase)(
          object,
        );
      },
    ].reduce((acc, fn) => fn(acc), { id, type });
    this.#builder.push(
      this.#appendProperty("verificationMethod", verificationMethod),
    );
    return this;
  }

  renderLD() {
    return this.#builder.reduce((acc, fn) => fn(acc), {});
  }
}
