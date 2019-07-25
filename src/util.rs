use crate::dep_types::Version;
use std::str::FromStr;
use std::{env, path::PathBuf, process, thread, time};

/// A convenience function
pub fn abort(message: &str) {
    {
        println!("{}", message);
        process::exit(1)
    }
}

pub fn possible_py_versions() -> Vec<Version> {
    vec![
        "2.0", "2.1", "2.2", "2.3", "2.4", "2.5", "2.6", "2.7", "3.3", "3.4", "3.5", "3.6", "3.7",
        "3.8", "3.9", "3.10", "3.11", "3.12",
    ]
    .into_iter()
    .map(|v| Version::from_str(v).unwrap())
    .collect()
}

pub fn venv_exists(venv_path: &PathBuf) -> bool {
    (venv_path.join("bin/python").exists() && venv_path.join("bin/pip").exists())
        || (venv_path.join("Scripts/python").exists() && venv_path.join("Scripts/pip").exists())
}

/// Checks whether the path is under `/bin` (Linux generally) or `/Scripts` (Windows generally)
/// Returns the primary bin path (ie under the venv), and the custom one (under `Lib`) as a Tuple.
pub fn find_bin_path(vers_path: &PathBuf) -> (PathBuf, PathBuf) {
    // The bin name should be `bin` on Linux, and `Scripts` on Windows. Check both.
    // Locate bin name after ensuring we have a virtual environment.
    // It appears that 'binary' scripts are installed in the `lib` directory's bin folder when
    // using the --target arg, instead of the one directly in the env.

    if vers_path.join(".venv/bin").exists() {
        (vers_path.join(".venv/bin"), vers_path.join("lib/bin"))
    } else if vers_path.join(".venv/Scripts").exists() {
        // todo: Perhasp the lib path may not be the same.
        (
            vers_path.join(".venv/Scripts"),
            vers_path.join("lib/Scripts"),
        )
    } else {
        println!("{:?}", vers_path);
        // todo: YOu sould probably propogate this as an Error instead of handlign here.
        abort("Can't find the new binary directory. (ie `bin` or `Scripts` in the virtual environment's folder)");
        (vers_path.clone(), vers_path.clone()) // Never executed; used to prevent compile errors.
    }
}

/// Wait for directories to be created; required between modifying the filesystem,
/// and running code that depends on the new files.
pub fn wait_for_dirs(dirs: &Vec<PathBuf>) -> Result<(), crate::AliasError> {
    // todo: AliasError is a quick fix to avoid creating new error type.
    let timeout = 1000; // ms
    for _ in 0..timeout {
        let mut all_created = true;
        for dir in dirs {
            if !dir.exists() {
                all_created = false;
            }
        }
        if all_created {
            return Ok(());
        }
        thread::sleep(time::Duration::from_millis(10));
    }
    Err(crate::AliasError {
        details: "Timed out attempting to create a directory".to_string(),
    })
}

/// Sets the `PYTHONPATH` environment variable, causing Python to look for
/// dependencies in `__pypackages__`,
pub fn set_pythonpath(lib_path: &PathBuf) {
    env::set_var(
        "PYTHONPATH",
        lib_path
            .to_str()
            .expect("Problem converting current path to string"),
    );
}
