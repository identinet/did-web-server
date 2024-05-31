# Changelog

All notable changes to this project will be documented in this file.

## [0.3.0] - 2024-05-31

### Added Features

- [f91cfd5](https://github.com/identinet/did-web-server/commit/f91cfd5a47332a1e7fb70969a4104c35804d81f6) Create leading directories before storing DID document

### Bug Fixes

- [**breaking**] [14ae074](https://github.com/identinet/did-web-server/commit/14ae0745393a8bcf4c10897f38d790cd9c2402ad) Revert back to default port 8000
- [**breaking**] [7b6957c](https://github.com/identinet/did-web-server/commit/7b6957cfe5b5d37309acd0873bd42ba5f693a9c0) Require proof purpose "authentication"
- [**breaking**] [ae7cd08](https://github.com/identinet/did-web-server/commit/ae7cd0874ada927759c26ee4d38946a613f0c68d) Change support characters in path segments/identifier
- [862206e](https://github.com/identinet/did-web-server/commit/862206e9aeb1d4364d50f0cb6a7de46eb7fe5135) Correct compiler warning
- [c5cf1a7](https://github.com/identinet/did-web-server/commit/c5cf1a704f6adc19a54138f54bce7d3e9212dc53) Enable TLS functionality

### Documentation

- [8ddfcac](https://github.com/identinet/did-web-server/commit/8ddfcacfb058f8bb5e211f8dcba109d7a27dccce) Correct rocket linke in the configuration section
- [eddd7e0](https://github.com/identinet/did-web-server/commit/eddd7e09cfbfeff702d1380453e46edf488e9716) Optimize configuration descriptions
- [2b97a4a](https://github.com/identinet/did-web-server/commit/2b97a4a2167d7b40a8c8b98e2e2ac1c1846bdf38) Add DID management documentation
- [c48d3c9](https://github.com/identinet/did-web-server/commit/c48d3c999f40009da6fd6dcc42a06c989491a0d9) Correct typos

### Miscellaneous Tasks

- [728292a](https://github.com/identinet/did-web-server/commit/728292ad0db90ce3891e7a9a4a11dabd2d762128) Update rocket dependency
- [a7bcdf5](https://github.com/identinet/did-web-server/commit/a7bcdf571c2807624a82e0fbf9373c246a1d26e0) Make error messages more expressive
- [4627a3d](https://github.com/identinet/did-web-server/commit/4627a3dac0fd2fcfa23c990e90c23141274d767e) Print DIDs found in submitted document

## [0.2.0] - 2024-05-30

### Added Features

- [bbc2db6](https://github.com/identinet/did-web-server/commit/bbc2db6b2cf7e0ef17f82086b6434002cd78ded7) Return different error codes depending on the result
- [59a8b92](https://github.com/identinet/did-web-server/commit/59a8b927f57aeefc5471cff6d90a6922d5db2bdc) Use environment variables to configure the server
- [f5e7bb3](https://github.com/identinet/did-web-server/commit/f5e7bb3bf359ffc8d60f3ca7a8c975ac8c2261cf) Implement generic log method
- [7df6413](https://github.com/identinet/did-web-server/commit/7df641395f00a54c60c7975f89e8e2833e825c11) Test DID id to match computed DID before storing it
- [5124140](https://github.com/identinet/did-web-server/commit/512414050fe3b04847d817ec1cdd06833f4af0a5) Implement port for did:web
- [2edbb95](https://github.com/identinet/did-web-server/commit/2edbb95aea5dae1b27d3e23b8e3d52949de9c3a3) Add tests for did crate
- [a96cbe3](https://github.com/identinet/did-web-server/commit/a96cbe307577ef4b61521c232ce1af32fae95efc) Implement deletion of DIDs
- [fbb5dc4](https://github.com/identinet/did-web-server/commit/fbb5dc49a056b6e903587b4e266bdc8cce4e2a0d) Implement support for .well-known/did.json
- [0dc1b21](https://github.com/identinet/did-web-server/commit/0dc1b215b76b94797ef159dc783faafa10cac209) Implement proofParameters query parameter
- [4498f06](https://github.com/identinet/did-web-server/commit/4498f068fa67965da5424f82a32eeefa9c5b4e38) Integrate with universal-resolver and implement update method
- [499c0d2](https://github.com/identinet/did-web-server/commit/499c0d277ca421ee821ed1db1cea9b91af69e890) Implement and test presentation verification
- [65b2c2b](https://github.com/identinet/did-web-server/commit/65b2c2bf0744e953136bbb6e7dc6f6bfb1717e30) Implement update
- [c91a3a6](https://github.com/identinet/did-web-server/commit/c91a3a6d4972c0402d6a2949aa261a54514272ec) Return DID if it already exists
- [77968e9](https://github.com/identinet/did-web-server/commit/77968e94773a7af527ff8d02a148adde3ef3f991) Add in-memory backend
- [01db9d7](https://github.com/identinet/did-web-server/commit/01db9d7aaaa7a2177346e7947c911dab4442845f) Add built-in resolver
- [e76d898](https://github.com/identinet/did-web-server/commit/e76d89870ce1dc70e974d49ab2055cf116e7d5ef) Add option for ownership DID
- [7992a73](https://github.com/identinet/did-web-server/commit/7992a738d921b6411bd8a78780115b279a44f74c) Add first implementation of a client
- [61f6aa9](https://github.com/identinet/did-web-server/commit/61f6aa9d86ae1247f28656f93b4081269b18e5ed) Move log to sanctuary and add RequestType
- [0e564cf](https://github.com/identinet/did-web-server/commit/0e564cfbfa080ac0473af8f3c3347ba01e3303e0) Implement request builder
- [e7aea25](https://github.com/identinet/did-web-server/commit/e7aea25e3ce759847525ebb23dfb2c7870a00a85) Implement deactivate function
- [81aebfa](https://github.com/identinet/did-web-server/commit/81aebfa4704c42673c933c1ab0746f2420d1f38a) Add verifiable presentation tools
- [58ddbf0](https://github.com/identinet/did-web-server/commit/58ddbf0331dba849b76dcfa3315ed549c3758f0c) Add logJson utility
- [ab3e380](https://github.com/identinet/did-web-server/commit/ab3e3802ea24c124f828e5ef93fb45939c89470b) Implement delete method
- [943f269](https://github.com/identinet/did-web-server/commit/943f26994008a9fe9e5fe5dd4155e8d1a1a5738f) Externalize verify_presentation implementation
- [58af25b](https://github.com/identinet/did-web-server/commit/58af25bb67921a91c9a7657a9faa4ecd8716b4a6) Verify that the DID is the subject in the DID Doc
- [64c8838](https://github.com/identinet/did-web-server/commit/64c8838e2842bff4fc51667a76921229cdd4c3e1) Add support for create authentication
- [64b1db5](https://github.com/identinet/did-web-server/commit/64b1db572aa283b8d7067e01cdb4210193f26d4d) Implement authentication for delete endpoint
- [95dc1f6](https://github.com/identinet/did-web-server/commit/95dc1f68c55fcbac137bbb6858c4fb620546a584) Enable local search
- [**breaking**] [ab14d73](https://github.com/identinet/did-web-server/commit/ab14d738a0d368d32e3c8b58fb846da387ae7f28) Add support for did:jwk, remove support for did:webkey and did:ethr

### Bug Fixes

- [ffdbf71](https://github.com/identinet/did-web-server/commit/ffdbf71b63a95f22ddf2f3f941bca1c9b10ebdb2) Correct linter issues
- [**breaking**] [0a0902a](https://github.com/identinet/did-web-server/commit/0a0902aec471b9c2beb31ce910780661aaf97e4e) Change file store to reflect the exact structure of did:web
- [35e7a5e](https://github.com/identinet/did-web-server/commit/35e7a5e6330307aa67d220c14df17dcd004af513) Add missing fixtures
- [4938c17](https://github.com/identinet/did-web-server/commit/4938c17c7b2b434ef9734f821acf6dd0a4911b29) Change environment variable prefix to DWS_
- [a785c7b](https://github.com/identinet/did-web-server/commit/a785c7b9fc827925c7d8fffdee60100b47302b76) Correct typos in openapi spec
- [71a6e1d](https://github.com/identinet/did-web-server/commit/71a6e1d27c1af9bab38807be66b596f17d3a719a) Use of default port for tests
- [b361d23](https://github.com/identinet/did-web-server/commit/b361d236493a912266baef5237b4a1fe33d437e9) Remove dead code and remove duplicate documentation

### Documentation

- [58ada32](https://github.com/identinet/did-web-server/commit/58ada329bc19ed88a79f842ab096a7a1a920494c) Add documentation
- [a896838](https://github.com/identinet/did-web-server/commit/a896838d743993f429d33fd3d97d7180b38411f4) Update link to SSI lib
- [d0265d7](https://github.com/identinet/did-web-server/commit/d0265d744761db1bb69c332e401de593be3a9b17) Add initial page with API docs
- [e1e91e1](https://github.com/identinet/did-web-server/commit/e1e91e17b9db089fc696403e319adf0d6acb0faf) Add first draft of documentation structure
- [b6a07ce](https://github.com/identinet/did-web-server/commit/b6a07ce151ff524ee13b6d6739f1a4f460e913e7) Unify naming of operations
- [20f8fb1](https://github.com/identinet/did-web-server/commit/20f8fb126eedd15006a5e066b29e7386d4a955ae) Add getting started documentation and migrate configuration docs
- [31a443a](https://github.com/identinet/did-web-server/commit/31a443a70d39ba18303169891b33fbcb68af0448) Migrate to starlight
- [95455bd](https://github.com/identinet/did-web-server/commit/95455bd05442084636bb5a4e63992f5af2cfecbf) Correct favicon
- [73fd85c](https://github.com/identinet/did-web-server/commit/73fd85c29630b79cc74a050a04e35a0eadf4a540) Add reference to writing how-to guides
- [5542e09](https://github.com/identinet/did-web-server/commit/5542e0981071ab2e4fde936187591d8928a65ef5) Update getting started guide
- [ec05a3d](https://github.com/identinet/did-web-server/commit/ec05a3d32d9172a6c078da89d0c291cd134c89a9) Align openapi documentation with implementation
- [0ac0b2f](https://github.com/identinet/did-web-server/commit/0ac0b2f5e59030052011c644a65f383ad20e0c67) Update API documentation and styles
- [2acfc9a](https://github.com/identinet/did-web-server/commit/2acfc9a945e92495e4088d648bc45f2b41e138a7) Describe how to set up a local test server
- [6d8994a](https://github.com/identinet/did-web-server/commit/6d8994ab480bec7648b4d24e382eded7cc78ed2b) Update examples to use new didkit-cli image
- [0da7c4c](https://github.com/identinet/did-web-server/commit/0da7c4ca18aa1c82ef944d30104f24ed74abd032) Update service description
- [3ccb970](https://github.com/identinet/did-web-server/commit/3ccb9703e6a14778461b7d63757f9e4f1bfb2e8b) Readd docs repository
- [e08399c](https://github.com/identinet/did-web-server/commit/e08399c6479ce6c1102977032d2f32ed606df470) Cleanup documentation
- [91f8263](https://github.com/identinet/did-web-server/commit/91f8263781bc31af5b770bdf5452cfa407f51120) Add congratulations page
- [a423bb7](https://github.com/identinet/did-web-server/commit/a423bb7c470a4144c39225887c93b9a0eaff8bc7) Document architecture diagram
- [da2f4b1](https://github.com/identinet/did-web-server/commit/da2f4b1f47d86c798a3e965afa0d8a61f89985b7) Add component diagram that shows the process for updating a DID document
- [a1a1b9c](https://github.com/identinet/did-web-server/commit/a1a1b9c9b7fdfd8c25fb2d6ce6826b8bc9eff3d5) Correct response code of create method

### Miscellaneous Tasks

- [f1e1b51](https://github.com/identinet/did-web-server/commit/f1e1b5105a3614c4a4392fe3cd430d4da72f8a37) Add experiments
- [5d8a938](https://github.com/identinet/did-web-server/commit/5d8a93819549d67831d066010f9deb167fec4d35) Update Justfiles
- [f804159](https://github.com/identinet/did-web-server/commit/f8041593e862d833b8990f9b926222b530f4750e) Support JSON return values
- [bd0818c](https://github.com/identinet/did-web-server/commit/bd0818c768dbec50f5b6dc483c825d26f4e9c6ba) Implement tests
- [e2db790](https://github.com/identinet/did-web-server/commit/e2db790d68ca589d8be21597f9be439c3b8b8398) Implement json extension
- [473a5be](https://github.com/identinet/did-web-server/commit/473a5be57c27251248f1231eae1db736913e9561) Use compute_filename in get method
- [f2268f4](https://github.com/identinet/did-web-server/commit/f2268f4771df5292850a1bb15d2f6dd3fb1e9b70) Remove custom result
- [8fdcc01](https://github.com/identinet/did-web-server/commit/8fdcc01a34f537c2e2356af1daff6b0122f8aa1f) Add documentation target
- [01946fb](https://github.com/identinet/did-web-server/commit/01946fbda6a23c01b48ff49c90d3c0a8cf12c8b7) Replace custom DIDDoc with ssi::did::Document
- [a7e98c3](https://github.com/identinet/did-web-server/commit/a7e98c3a063e6a3ac3a7532f35f5b0bbd56f6d2c) Separate crate into multiple libs
- [cfb0d66](https://github.com/identinet/did-web-server/commit/cfb0d66c8c800a7daa0a260c1a28153c5bd5c157) Make get return application/did+json content type
- [a2c22f6](https://github.com/identinet/did-web-server/commit/a2c22f67be8678e6f34c0e770ad499fc4c421486) Migrate from &str to PathBuf for id and base_dir
- [f44be12](https://github.com/identinet/did-web-server/commit/f44be127ccd114d80febfc63f52b7083e4201a65) Limit watch to the src folder
- [b827303](https://github.com/identinet/did-web-server/commit/b827303ee5c8eb49b92215b2d46364c56a8b2a80) Implement custom DIDContentTypes
- [8fbd92f](https://github.com/identinet/did-web-server/commit/8fbd92fcf7ea07f4b41d9992cc1bb58bf8d0a934) Implement clippy suggestions
- [0df0b74](https://github.com/identinet/did-web-server/commit/0df0b74a1715f4b8bb8f2bf963b81a3b282b9810) Simplify DIDSegment implementation
- [e885048](https://github.com/identinet/did-web-server/commit/e8850487aa2e388bdabc3ebd7332d9decef5a97c) Rename project to did-web-server
- [7e450b6](https://github.com/identinet/did-web-server/commit/7e450b6190afbd9f2cfe3463912ad25fd8454959) Fix clippy warnings
- [17463e8](https://github.com/identinet/did-web-server/commit/17463e8c31f2f7ce7a195980badc89fb50782712) Implement Default for Config
- [b5b0d14](https://github.com/identinet/did-web-server/commit/b5b0d14f64646e06ca4c62c3bb606f433f03c1f5) Externalize utils
- [27000f0](https://github.com/identinet/did-web-server/commit/27000f0b386c219070373d92e0479d0e3d9c5b22) Reject expired DID Doc credentials
- [db2e947](https://github.com/identinet/did-web-server/commit/db2e9472b6bc5e8a44c6d3985822da9901d50d74) Unsure missing DIDDoc credential returns an error
- [9b08ec4](https://github.com/identinet/did-web-server/commit/9b08ec47ff4ec95c2767678a4524925f1a031311) Ensure error is returned if id in DID Doc doesn't match
- [**breaking**] [80e67e1](https://github.com/identinet/did-web-server/commit/80e67e1f5afc885dac29f4f4b351b08fdbd43659) Remove /v1/web prefix for API endpoints
- [35a41d1](https://github.com/identinet/did-web-server/commit/35a41d1d25e18f145ea2eaf7d3f980bd31325872) Extract VC helper logic into utility functions
- [a8c0deb](https://github.com/identinet/did-web-server/commit/a8c0debd96eea09491becb60a938b2907a6fffce) Extract proof verification into utils function
- [475b6b4](https://github.com/identinet/did-web-server/commit/475b6b4ada97a9e4be928e6ec16f676bf2fda746) Restructure library modules
- [7cbabdd](https://github.com/identinet/did-web-server/commit/7cbabdd5e8fdc40ca1af4127d1d32a907f7f6b56) Move back to esm.sh
- [e0c1484](https://github.com/identinet/did-web-server/commit/e0c148420aec64cc4b6883af59f785f8b003fa0d) Update and break dependencies
- [7d4aa6b](https://github.com/identinet/did-web-server/commit/7d4aa6ba448434804e6fb81f5a7ceaff242c758f) Restructure types and start implementation of e2e tests
- [e575df8](https://github.com/identinet/did-web-server/commit/e575df81452c07086de7daa2a6a38ba2eeb02031) Update to latest ssi library version
- [0d501bf](https://github.com/identinet/did-web-server/commit/0d501bfed51e2e8cae0baf53c2bf5def878fa374) Remove unused documents in did_store
- [c07bb2f](https://github.com/identinet/did-web-server/commit/c07bb2fa7f36aa89174294004c674d70f7ed0e2d) Disable duplicate DID verification
- [1372959](https://github.com/identinet/did-web-server/commit/1372959e874b1b45f09347d42d6b0460d9c98f14) Update styles
- [1717740](https://github.com/identinet/did-web-server/commit/17177401307eb920d30442cdddd06a5d338f311c) Set default server port to 3000
- [20572c8](https://github.com/identinet/did-web-server/commit/20572c85aaeb22361aa88d22dfba3a0fe34cdcc2) Set default server port to 3000
- [1edd677](https://github.com/identinet/did-web-server/commit/1edd6775e4e03de12c2ad2303ae56df64a82fdfa) Update dependencies
- [00b3a49](https://github.com/identinet/did-web-server/commit/00b3a497c3b6cb92d00062887d68ddfe6c32d550) Remove cargo configuration from repository

### Other

- [4e00cf7](https://github.com/identinet/did-web-server/commit/4e00cf75af7ccffec16dfaa5f08676c08fd43ffb) Add build cache and make lint fail if there are warnings
- [bf853f4](https://github.com/identinet/did-web-server/commit/bf853f45c814b74ac8f51a66116db20b0caae2d8) Add pre-commit hook
- [88b88fc](https://github.com/identinet/did-web-server/commit/88b88fce48f00e0fe6034739c7b1c51d21da7009) Make warnings fail the build
- [b4a42df](https://github.com/identinet/did-web-server/commit/b4a42df4ad9262a387bc687ac34cf1e3ec0f4846) Define target folder in variable

### Testing

- [9d14723](https://github.com/identinet/did-web-server/commit/9d147238b4ea397a04f7c93c37ed62df2a85cf92) Add integration tests for public API
- [36101b6](https://github.com/identinet/did-web-server/commit/36101b66d70817e07ac8dc4942c687e5334fd4b7) Externalize tests
- [8993a4b](https://github.com/identinet/did-web-server/commit/8993a4b6dd78cf4e696f2895533ee58a394f8232) Implement multiple resolvers and test invalid holder

<!-- generated by git-cliff -->
