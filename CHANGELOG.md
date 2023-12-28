## 1.0.0-beta.1 (2023-12-28)


### ⚠ BREAKING CHANGES

* **VMBuilder:** closes #3
* **Machine:** 
* **Program:** closes #2
* 
* **test_get_bit:** 

### Features

* add an ASCII Table struct ([dd2a6e6](https://github.com/AliSajid/BrainFoamKit/commit/dd2a6e66f5738775715437140efe11ab5750f531))
* add placeholder main() ([36b6a70](https://github.com/AliSajid/BrainFoamKit/commit/36b6a701a777b4a16c34e2dc2896d150456e1e02))
* add pretty table for the ascii table output ([c0a5aae](https://github.com/AliSajid/BrainFoamKit/commit/c0a5aae5e7bfbd8ab908bb56ae045bc07fd067c7))
* **binary:** add placeholder TUI for the interpreter ([de30d66](https://github.com/AliSajid/BrainFoamKit/commit/de30d66d65ed294e1009c89921e5464a20dddc9b))
* **bit:** add a bit struct and associated functions ([e44dffd](https://github.com/AliSajid/BrainFoamKit/commit/e44dffdeb3635f3fcf24b791aaad64d6d8e3725f))
* **Bit:** impl the Bit*Assign traits ([8dba984](https://github.com/AliSajid/BrainFoamKit/commit/8dba9846e4509d0ddbe51b5c1c26a3d672f336db))
* **bit:** implement BitOr, BitAnd and BitXor traits for Bit ([7b0d39a](https://github.com/AliSajid/BrainFoamKit/commit/7b0d39a35631b446f7d8d0b1100da8b9850668b5))
* **BrainFoamKitMachine:** add a rudimentary BrainFoamKitMachine ([1d333bd](https://github.com/AliSajid/BrainFoamKit/commit/1d333bd8604a36212a0e5a2c627d076ae84a6a4b))
* **Byte:** add a byte struct and associated implementation ([2a9c7be](https://github.com/AliSajid/BrainFoamKit/commit/2a9c7be8f0c7605a84133c5fc87896b65f4cb085))
* **byte:** add abyte struct and associated functions ([5ee80c4](https://github.com/AliSajid/BrainFoamKit/commit/5ee80c4d15c048306f4aa97d18e95317da20a846))
* **Byte:** add an iterable type for byte ([b0f08a8](https://github.com/AliSajid/BrainFoamKit/commit/b0f08a805de8171000e3de22d5f4511b6f6ba8dd))
* **Byte:** implement `IntoIterator` for `&Byte` ([a814022](https://github.com/AliSajid/BrainFoamKit/commit/a814022e2828e731db47c3fdafb021c8392620df))
* **Byte:** implement basic Byte functionality ([6e51f2c](https://github.com/AliSajid/BrainFoamKit/commit/6e51f2c5206b7245b3424bf5bae160ca8c6baf4a))
* expose `Program` and `BrainFoamKitMachine` from the library ([ce3790e](https://github.com/AliSajid/BrainFoamKit/commit/ce3790ef9b003b6758420fa84a3a67f596b4167b))
* implement Not for Bit ([afe5066](https://github.com/AliSajid/BrainFoamKit/commit/afe506625030aaf435cc6cdc404e92af65580276))
* **instruction:** add an instruction enum to manage and read instructions ([8b3bd89](https://github.com/AliSajid/BrainFoamKit/commit/8b3bd898956b78c3d966be6f9646c53bcef15ac6))
* **Instruction:** add instruction generation from chars ([f938c88](https://github.com/AliSajid/BrainFoamKit/commit/f938c88e79118ec755e77e369a31218b4709d0dd))
* **Instruction:** change the representation of instructions ([cf0de0a](https://github.com/AliSajid/BrainFoamKit/commit/cf0de0aaba56c8cbb3ee70d5c896f5959fd2f18c))
* **lib:** add the basic bit, byte and tape structs ([ab9458d](https://github.com/AliSajid/BrainFoamKit/commit/ab9458d868afd42156e2fad6b7f27e1122c8e6b6))
* **nybble:** add a nybble and associated functions ([e3e959e](https://github.com/AliSajid/BrainFoamKit/commit/e3e959ebc58d8fe4839233a095764aa5779c7ae4))
* **Nybble:** add increment and decrement functions ([a66d4a8](https://github.com/AliSajid/BrainFoamKit/commit/a66d4a8f603f5fd4cac80ff721ba47c80f7927bc))
* **nybble:** add individual bit getter ([e7557f0](https://github.com/AliSajid/BrainFoamKit/commit/e7557f0a33548e8a6994f0090b759403f3c5e9f7))
* **Nybble:** add iterator implementation for Nybble ([16b0e73](https://github.com/AliSajid/BrainFoamKit/commit/16b0e739638955bdde6678d200c2f9c86a0157b5))
* **nybble:** add nybble functionality ([43edddf](https://github.com/AliSajid/BrainFoamKit/commit/43edddface3c898464866317f95ca70717196ae2))
* **Nybble:** add stubs for increment() and decrement() for Nybble ([d137203](https://github.com/AliSajid/BrainFoamKit/commit/d1372036288baba0bfc3db567b7435e5621367c6))
* **nybble:** immplement Not trait for Nybble ([5be421a](https://github.com/AliSajid/BrainFoamKit/commit/5be421ae9b57bbf158346c09a51b088a9abb69a2))
* **Nybble:** implement `IntoIterator` for `Nybble` ([043b687](https://github.com/AliSajid/BrainFoamKit/commit/043b687f83c33c5138b72c8963150a6da82344a0))
* **Nybble:** refactor to iterable nybble ([fdc440e](https://github.com/AliSajid/BrainFoamKit/commit/fdc440eddebcde11bed87dc7fddf0c4b55340126))
* **Program:** add an index method ([0910507](https://github.com/AliSajid/BrainFoamKit/commit/0910507f8832780985e6b03dc9ceef2b1351bdbc))
* **Program:** add the `Program` struct ([dbcb33d](https://github.com/AliSajid/BrainFoamKit/commit/dbcb33d68c82291f368f9b08ebaa0f30c1a5bf5f))
* redo the bfkrun binary for alpha-testing ([314391c](https://github.com/AliSajid/BrainFoamKit/commit/314391c2bbf7246e42e4ea90d09a8685e4c455ba))
* testing new run ([f534933](https://github.com/AliSajid/BrainFoamKit/commit/f53493325b592037808cd5efababce09ac9acf88))
* use ratatui for TUI building ([9177748](https://github.com/AliSajid/BrainFoamKit/commit/91777480299f6d9c7ea3869eff49006c271c5d85))
* **VMBuilder:** add a `Builder` interface to the Virtual Machine ([c83d8cb](https://github.com/AliSajid/BrainFoamKit/commit/c83d8cb3e073aabf16001c9a50d101300124df5f))


### Bug Fixes

* fix several failing tests due to inconsistent documentation ([557e2a4](https://github.com/AliSajid/BrainFoamKit/commit/557e2a4573b3a2c70131f73be565b68aac803bec))
* **interpreter:** change the upperlimit of the ASCII sequence ([6cf17ec](https://github.com/AliSajid/BrainFoamKit/commit/6cf17ec8abfc511ec7e4b184cfd330e4155311b5))
* **interpreter:** spell check ([39aeec6](https://github.com/AliSajid/BrainFoamKit/commit/39aeec6647a34156550fea8f48992d377445a60e))
* remove generic type parameter ([5270bf2](https://github.com/AliSajid/BrainFoamKit/commit/5270bf2ac4a126b58a80ee9e829c06aa645f9385))
* update the doctest to pass ([ded4ad1](https://github.com/AliSajid/BrainFoamKit/commit/ded4ad1ae7a77df542863764853ad0c8726f6fb3))
* update the pacakge list for the gitpod dockerfile ([cd9bd25](https://github.com/AliSajid/BrainFoamKit/commit/cd9bd255173b7cc2d44c60ab1b4a643d1dcb2694))
* **VirtualMachine:** update `increment_value()` and `decrement_value()` to do in-place mutation ([822b8cb](https://github.com/AliSajid/BrainFoamKit/commit/822b8cb89fcce0fab0ca0ce467ffd8da1fade07a))


### Code Refactoring

* **Machine:** basic refactor of the Machine struct before doing an overhaul ([86649c5](https://github.com/AliSajid/BrainFoamKit/commit/86649c5bc4d12c925a9659f8cef7b7243b06e03c))
* **Program:** change `Program` to be an immutable container of instructions ([91c6187](https://github.com/AliSajid/BrainFoamKit/commit/91c618779c5c63de7bb5845a14cb6ccf7d495344))


### Tests

* **test_get_bit:** fix the bit location for the test ([ec656ae](https://github.com/AliSajid/BrainFoamKit/commit/ec656aee31e92193a9101efca00f67f9bebd9590))

## [1.0.0-alpha.13](https://github.com/AliSajid/BrainFoamKit/compare/v1.0.0-alpha.12...v1.0.0-alpha.13) (2023-12-28)


### Bug Fixes

* **VirtualMachine:** update `increment_value()` and `decrement_value()` to do in-place mutation ([822b8cb](https://github.com/AliSajid/BrainFoamKit/commit/822b8cb89fcce0fab0ca0ce467ffd8da1fade07a))

## [1.0.0-alpha.12](https://github.com/AliSajid/BrainFoamKit/compare/v1.0.0-alpha.11...v1.0.0-alpha.12) (2023-12-28)


### Bug Fixes

* remove generic type parameter ([5270bf2](https://github.com/AliSajid/BrainFoamKit/commit/5270bf2ac4a126b58a80ee9e829c06aa645f9385))

## [1.0.0-alpha.11](https://github.com/AliSajid/BrainFoamKit/compare/v1.0.0-alpha.10...v1.0.0-alpha.11) (2023-12-27)


### Bug Fixes

* update the pacakge list for the gitpod dockerfile ([cd9bd25](https://github.com/AliSajid/BrainFoamKit/commit/cd9bd255173b7cc2d44c60ab1b4a643d1dcb2694))

## [1.0.0-alpha.8](https://github.com/AliSajid/BrainFoamKit/compare/v1.0.0-alpha.7...v1.0.0-alpha.8) (2023-08-26)


### Features

* **Byte:** implement basic Byte functionality ([6e51f2c](https://github.com/AliSajid/BrainFoamKit/commit/6e51f2c5206b7245b3424bf5bae160ca8c6baf4a))
