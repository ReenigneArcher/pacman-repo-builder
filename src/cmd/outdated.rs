use super::super::{
    args::{OutdatedArgs, OutdatedDetails},
    manifest::Repository,
    utils::{outdated_packages, DbInit, DbInitValue, PackageFileName},
};
use std::{fs::read_dir, path::PathBuf};

pub fn outdated(args: OutdatedArgs) -> i32 {
    let OutdatedArgs { details } = args;
    let details = details.unwrap_or_default();

    let mut db_init = DbInit::default();
    let DbInitValue {
        manifest,
        database,
        mut error_count,
    } = match db_init.init() {
        Err(error) => return error.code(),
        Ok(value) => value,
    };

    let latest_packages: Vec<_> = database
        .package_file_base_names()
        .filter_map(|item| match item {
            Err(error) => {
                eprintln!("error in pkgbase of {}: {}", error.pkgbase, error.message);
                error_count += 1;
                None
            }
            Ok(value) => Some(value),
        })
        .collect();

    for repository in manifest
        .resolve_members()
        .filter_map(|member| {
            if let Some(repository) = member.repository {
                Some(repository)
            } else {
                eprintln!(
                    "(warning) a member with directory {:?} has no repositories",
                    member.directory,
                );
                None
            }
        })
        .flat_map(|repository| match repository {
            Repository::Single(path) => vec![path],
            Repository::Multiple(paths) => paths,
        })
    {
        let directory = if let Some(parent) = repository.parent() {
            parent
        } else {
            eprintln!("repository cannot be a directory: {:?}", repository);
            error_count += 1;
            continue;
        };

        // PROBLEM: read_dir cannot read "" as a directory
        // WORKAROUND: replace it with "."
        let valid_current_directory = PathBuf::from(".");
        let directory = if directory.as_os_str().is_empty() {
            &valid_current_directory
        } else {
            directory
        };

        let entries = match read_dir(directory) {
            Err(error) => {
                eprintln!("cannot read {:?} as a directory: {}", directory, error);
                error_count += 1;
                continue;
            }
            Ok(entries) => entries,
        };

        let mut current_packages = Vec::new();
        for entry in entries {
            let file_name = match entry {
                Err(error) => {
                    eprintln!(
                        "cannot read an entry of directory {:?}: {}",
                        directory, error
                    );
                    error_count += 1;
                    continue;
                }
                Ok(entry) => entry.file_name(),
            };

            if let Some(name) = file_name.to_str() {
                current_packages.push(name.to_string())
            } else {
                eprintln!("cannot convert {:?} to UTF-8", file_name);
                error_count += 1;
            }
        }

        for (
            ref file_name,
            PackageFileName {
                pkgname,
                version,
                arch,
            },
        ) in outdated_packages(&latest_packages, &current_packages)
        {
            match details {
                OutdatedDetails::PkgFilePath => {
                    println!("{}", directory.join(file_name).to_string_lossy());
                }
                OutdatedDetails::LossyYaml => {
                    println!("---");
                    println!("repository-file: {}", repository.to_string_lossy());
                    println!("repository-directory: {}", directory.to_string_lossy());
                    println!("file-name: {}", file_name);
                    println!("pkgname: {}", pkgname);
                    println!("version: {}", version);
                    println!("arch: {}", arch);
                }
                OutdatedDetails::StrictYaml => {
                    println!("---");
                    println!("repository-file: {:?}", repository);
                    println!("repository-directory: {:?}", directory);
                    println!("file-name: {:?}", file_name);
                    println!("pkgname: {:?}", pkgname);
                    println!("version: {:?}", version);
                    println!("arch: {:?}", arch);
                }
            }
        }
    }

    if error_count == 0 {
        0
    } else {
        eprintln!("{} errors occurred", error_count);
        1
    }
}
