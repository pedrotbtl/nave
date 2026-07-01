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

Archived; it moved to https://github.com/blockhouse-sec/nave
