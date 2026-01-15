<div align="center">
  <picture>
    <img src="./noir-logo.png" alt="The Noir Programming Language" width="35%">
  </picture>

[Website][Noir] | [Getting started] | [Documentation] | [Contributing]
</div>



# The Noir Programming Language

[![Non-deterministic fuzz tests](https://github.com/noir-lang/noir/actions/workflows/nightly-fuzz-test.yml/badge.svg)](https://github.com/noir-lang/noir/actions/workflows/nightly-fuzz-test.yml)

Noir is a Domain Specific Language for SNARK proving systems. It has been designed to use any ACIR compatible proving system.

**This implementation is in early development. It has not been reviewed or audited. It is not suitable to be used in production. Expect bugs!**

## NAVe (Noir Formal Verifier)

NAVe is a formal verifier for the Noir language. Not to be confused with a ZK verifier, NAVe is designed to check that the code gives rise to the expected constraints.
NAVe translates a ACIR program into a corresponding set of SMT constraints that can be verified by an SMT solver; NAVe relies on the Noir infrastructure to compile a
Noir program into ACIR.

A developer can annotate its Noir program with *verification asserts*. Unlike Noir builtin asserts, these asserts are not constraining the behaviour of the program,
they represent, instead, verification conditions that will be checked by the verified.

The main components of NAVe are in [tooling/acir_checker](tooling/acir_checker).

A simple tutorial for NAVe is in [test_programs/nave/tutorial/rps](test_programs/nave/tutorial/rps).

NAVe test programs are in [test_programs/nave](test_programs/nave).

The paper describing NAVe  --- and giving a formal semantics to (a subset of) ACIR --- is at: [https://arxiv.org/abs/2601.09372](https://arxiv.org/abs/2601.09372)

## Quick Start

Read the [installation section][Getting started] from the [Noir docs][Documentation].

Once you have read through the documentation, you can visit [Awesome Noir](https://github.com/noir-lang/awesome-noir) to run some of the examples that others have created.

## Getting Help

Join the Noir [forum][Forum] or [Discord][Discord]

## Contributing

See [CONTRIBUTING.md][CONTRIBUTING].

## Future Work

The current focus is to gather as much feedback as possible while in the alpha phase. The main focuses of Noir are _safety_ and _developer experience_. If you find a feature that does not seem to be in line with these goals, please open an issue!

## Minimum Rust version

This workspace's minimum supported rustc version is 1.85.0.

## License

Noir is free and open source. It is distributed under a dual license. (MIT/APACHE)

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this repository by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

[Noir]: https://www.noir-lang.org/
[Getting Started]: https://noir-lang.org/docs/getting_started/quick_start/
[Forum]: https://forum.aztec.network/c/noir
[Discord]: https://discord.gg/JtqzkdeQ6G
[Documentation]: https://noir-lang.org/docs/
[Contributing]: CONTRIBUTING.md
