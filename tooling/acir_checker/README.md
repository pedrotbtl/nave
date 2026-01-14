# NAVe: ACIR checker

To run NAVe one needs to install cvc5 with support to the theory of finite fields (ff). This can be done by downloading a release that corresponds to your environment from here https://github.com/cvc5/cvc5/releases/ --- the `-glp` options are built with the necessary ff options. The `cvc5` binary must be on your PATH. For instance, for a mac with arm one can download the following version [cvc5-macOS-arm64-static-gpl.zip](https://github.com/cvc5/cvc5/releases/download/cvc5-1.3.1/cvc5-macOS-arm64-static-gpl.zip).

## Command

You can use `nargo formal-verify` on a nargo project to check the main functions available using our checker. In the context of this project you can use `cargo run -- formal-verify` to run this command with the build nargo. For instance, you can navigate to one of the `NAVE` test programs and run
this command.

```
cd test_programs/nave/range/falsified/prf_11/a_3_add
cargo run -- formal-verify
```

### Configuration Options

The formal-verify command supports the following options:

1. --backend

Specifies the encoding used by NAVe. This option is represented as an enum with the following values:

a. ff-gb: Finite Field encoding using Grobner bases (default)

b. ff-split: Finite Field encoding with split constraints

c. int: Integer-based encoding

2. --relaxed

Controls whether the verifier runs in relaxed mode. When enabled, non-critical (non-breaking) translation or encoding errors are ignored, allowing the verifier to proceed on a best-effort basis.

Type: Boolean

Default: false

When set to true, the verifier attempts to translate and encode ACIR to SMT as robustly as possible, even in the presence of minor issue.

Example:

`cargo run -- formal-verify --backend=ff-split --relaxed`

## Test Programs

The Noir test programs have been evaluated with NAVe by manually inserting verification assertions.
These test cases are located in [test_programs/nave](test_programs/nave).

Directory Structure

The directory is organized based on whether the program includes a BLACKBOX::RANGE constraint and on the solver’s outcome:

1. range/:  Programs that contain a BLACKBOX::RANGE constraint

    verified/: Solver output is UNSAT

    falsified/: Solver output is SAT

2. no_range/: Programs that do not contain range constraints

    verified/: Solver output is UNSAT

    falsified/: Solver output is SAT