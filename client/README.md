# did-web Server JavaScript Client

This module provides an interface for creating, updating and retrieving DIDs
stored on the did-web server.

## CLI

```bash
deno run cli.js
```

## JavaScript

```javascript
import {create, update, delete, template, loadKey, ...} from "lib.js";

/* create */

/// Dieser Teil kommt aus den jeweiligen Signature Suites und braucht von mir nicht wirklich doll verstärkt zu werden
/// Ich bin gut beraten dies nicht zu verkomplizieren
/// Mehrere Arten von Keys werden unterstützt - Multibase EdDSA Keys und JWK keys. In der lib brauche ich nur wenige zu unterstützen
// 1. load server owner's private key
loadKeyFromFile(filename)
loadKey(string)
// 1. load owner's did
loadOwnerDID(did)
// the key suite contains the key and an ID
// when loading the key with a controller specified, the corresponding key ID will be generated
sign(text) // die kommt eigentlich von vc.js
suite = { issuerdid, issuerkey_reference, issuerkey }
// diese zusätzlichen Informationen müssen jha auch irgendwo gespeichert werden ....

// 2. load DID user's public key
loadPubKeyFromFile(filename)
loadPubKey(string)

// 3. create the new DID document for the DID user
createDIDDoc(didmethod(web), pubkey)

newdid = createDIDDoc(pubkeyUser, did)
existing_did = resolve(did) // or resolve existing DID and update the document

// 4. put the DID document in a VC and sign it with the owner's private key
createVCfromDiddoc(suite, diddoc)
// 5. fetch proof parameters?? - they could also be computed locally based upon the old/expected DID doc
fetchProofParameters(did:web-did)
// 6. put the VC in a verifiable presentation and sign it withthe owner's private key
createVP(suite, vc, proofparams) // use either vailidUntil in the credential or an ID that will be saved by the server to prevent replay attacks
// Retrieve a request for a presentation that's then filled out by the user's wallet; there's no standard for that
// 7. create the DID document on the server
createDIDonServer(vp)
createDIDonServer(suiteOwner, newDidDoc)
// Der Ablauf kann auch sein, dass der Onwer ein signiertes DIDDocVC ausstellt, das dann vom Nutzer via Presentation eingereicht wird
// Oder ich definiere ich eigenes VC, welches die Erstellung einer definierten DID erlaubt .. aber, der Nutzer hat noch keine DID, das macht es etwas schwierig. Kurzfristig könnte eine did:key verwendet werden, um die Nicht-existenz der DID zu überbrücken
// Bei diesem Schritt habe ich einige Fragen wie es denn genau ablaufen kann
// User generiert Key - okay
// Admin generiert Key - okay
// Wie wird der User Key in ein DID-Doc überführt? Optionen:
// 1. User schickt Admin eine Anfrage für eine DID - VC Issuing request (dies transportiert einen public key, der für weitere Aktivitäten genutzt werden kann) - permission to create did:web:domain.com:ABC (z.B. via mail); Admin signs request to allow user to create DID:web with they current DID:key; User sendet Anfrage an Server und erstellt eigene ID; Der VC kann auch das Recht beinhalten die DID wieder zu löschen. Alternativ kann dieses Recht separat angefordert werden;
// 2. Der Admin kann auch jederzeit selbst Lösch- und Erzeugungs-Requests an den Server schicken - if(admin) {do it}. Es bleibt dem Nutzer überlassen wie die KeyID zum Admin transportiert wird
// - Variante 1 geht in Richtung eines Berechtigungs-Management-Systems, das mittels VCs implementiert ist und komplett asynchron, ohne zentrale Services funktioniert. Das ist spannend zu implementieren, aktuell steht dies aber nicht im Vordergrund, da dafür ein ordentlicher Transportkanal a la DIDComm benötigt wird und ein Wallet, das die Interaktionen verarbeiten kann!

// https://github.com/digitalbazaar/vc/
// https://github.com/digitalbazaar/ed25519-signature-2020
// all the terminology is defined here: https://www.w3.org/community/reports/credentials/CG-FINAL-di-eddsa-2020-20220724/
// don't worry about the handling of cryptographic material. I'll get to it later
// [ ] move library code to lib/
// [ ] create diddoc creation method
// [ ] create DIDDoc to VC method that can be issued
// [ ] create fetch proof parameters method
// [ ] create interface method for creating a DID on the server

/* update */
// 1. load DID user's private key
// 2. create an updated DID document for the DID user, e.g. by fetching the existing document and changing it
// 3. put the DID document in a VC and sign it with the user's private key
// 4. fetch proof parameters
// 5. put the VC and the proof paramters in a verifiable presentation and sign it with the user's private key
// 6. update the DID document on the server
updateDID(vp)
updateDID(suiteUser, newDidDoc)

/* delete */
// 1. load DID owner's private key
// 2. (only necessary because the VP must conatain some VC) create any DID document,  e.g. by fetching the existing document
// 3. (only necessary because the VP must conatain some VC) put the DID document in a VC and sign it with the owner's private key
// 4. fetch proof parameters
// 5. put the VC and the proof paramters in a verifiable presentation and sign it with the owner's private key
// 6. delete the DID document on the server
deleteDID(vp)
deleteDID(suiteOwner, did)
```
