use acir_checker::{check_program, Output};
use clap::Args;
use nargo::workspace::Workspace;
use nargo_toml::PackageSelection;
use noir_artifact_cli::Artifact;
use noirc_driver::{CompileOptions, CompiledProgram};

use super::compile_cmd::compile_workspace_full;
use super::{LockType, PackageOptions, WorkspaceCommand};
use crate::errors::CliError;

/// Formally verify the functions of a compiled program using an SMT solver.
#[derive(Debug, Clone, Args)]
// #[clap(visible_alias = "c")]
pub(crate) struct FormalVerifyCommand {
    #[clap(flatten)]
    pub(super) package_options: PackageOptions,

    #[clap(flatten)]
    compile_options: CompileOptions,
}

impl WorkspaceCommand for FormalVerifyCommand {
    fn package_selection(&self) -> PackageSelection {
        self.package_options.package_selection()
    }

    fn lock_type(&self) -> LockType {
        // Compiles artifacts.
        LockType::Exclusive
    }
}

pub(crate) fn run(args: FormalVerifyCommand, workspace: Workspace) -> Result<(), CliError> {
    // Compile the full workspace in order to generate any build artifacts.
    let debug_compile_stdin = None;
    compile_workspace_full(&workspace, &args.compile_options, debug_compile_stdin)?;

    // Go over binary packages that have been created to verify their functions.
    let binary_packages = workspace.into_iter().filter(|package| package.is_binary());
    for package in binary_packages {
        let program_artifact_path = workspace.package_build_path(package);

        let artifact = Artifact::read_from_file(&program_artifact_path)?;
        let artifact_name = program_artifact_path.file_stem().and_then(|s| s.to_str()).unwrap_or_default();

        let (compiled_program, _circuit_name): (CompiledProgram, String) = match artifact {
            Artifact::Program(program) => (program.into(), artifact_name.to_string()),
            Artifact::Contract(_) => {
                return Err(CliError::Generic("Only works for Programs.".to_string()))
            }
        };

        let output = check_program(&compiled_program, false);
        match output {
            Output::Sat(model) => {
                println!("Sat with Model: {:?}", model);
            },
            Output::Unsat => {
                println!("Unsat");
            },
            Output::Unknown => {
                println!("Unknown");
            }
            
        }
    }
    Ok(())
}
