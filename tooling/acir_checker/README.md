# ACIR checker

To run the ACIR checker one needs to install cvc5 with support to the theory of finite fields (ff). This can be done by downloading a release that corresponds to your environment from here https://github.com/cvc5/cvc5/releases/ --- the `-glp` options are built with the necessary ff options. The `cvc5` binary must be on your PATH. For instance, for a mac with arm one can download the following version [cvc5-macOS-arm64-static-gpl.zip](https://github.com/cvc5/cvc5/releases/download/cvc5-1.3.1/cvc5-macOS-arm64-static-gpl.zip).

# Command

You can use `nargo formal-verify` on a nargo project to check the main functions available using our checker. In the context of this project you can use `cargo run -- formal-verify` to run this command with the build nargo. For instance, you can go into `test_programs/fuzzing_failure/array` and run this command.