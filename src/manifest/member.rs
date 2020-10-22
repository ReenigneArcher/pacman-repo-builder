use super::{
    Associations, BorrowedDirectory, BorrowedPackager, BorrowedPacman, BuildMetadata,
    GlobalSettings, OwnedDirectory, OwnedPackager, OwnedPacman, Wrapper,
};
use pipe_trait::*;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Member<Directory, Pacman, Packager>
where
    Directory: Associations + AsRef<Path>,
    Pacman: Associations + AsRef<str>,
    Packager: Associations + AsRef<str>,
{
    pub directory: Directory,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_build_metadata: Option<BuildMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_missing_dependencies: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clean_before_build: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clean_after_build: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_rebuild: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pacman: Option<Pacman>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packager: Option<Packager>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_failure: Option<bool>,
}

pub type OwnedMember = Member<OwnedDirectory, OwnedPacman, OwnedPackager>;
pub type BorrowedMember<'a> =
    Member<BorrowedDirectory<'a>, BorrowedPacman<'a>, BorrowedPackager<'a>>;

impl<Directory, Pacman, Packager> Member<Directory, Pacman, Packager>
where
    Directory: Associations + AsRef<Path>,
    Pacman: Associations + AsRef<str>,
    Packager: Associations + AsRef<str>,
{
    pub fn as_path(&self) -> BorrowedMember<'_> {
        BorrowedMember {
            directory: self.directory.as_ref().pipe(Wrapper::from_inner),
            read_build_metadata: self.read_build_metadata,
            install_missing_dependencies: self.install_missing_dependencies,
            clean_before_build: self.clean_before_build,
            clean_after_build: self.clean_after_build,
            force_rebuild: self.force_rebuild,
            pacman: self
                .pacman
                .as_ref()
                .map(AsRef::as_ref)
                .map(Wrapper::from_inner),
            packager: self
                .packager
                .as_ref()
                .map(AsRef::as_ref)
                .map(Wrapper::from_inner),
            allow_failure: self.allow_failure,
        }
    }

    pub fn to_path_buf(&self) -> OwnedMember {
        OwnedMember {
            directory: self
                .directory
                .as_ref()
                .to_path_buf()
                .pipe(Wrapper::from_inner),
            read_build_metadata: self.read_build_metadata,
            install_missing_dependencies: self.install_missing_dependencies,
            clean_before_build: self.clean_before_build,
            clean_after_build: self.clean_after_build,
            force_rebuild: self.force_rebuild,
            pacman: self
                .pacman
                .as_ref()
                .map(AsRef::as_ref)
                .map(ToString::to_string)
                .map(Wrapper::from_inner),
            packager: self
                .packager
                .as_ref()
                .map(AsRef::as_ref)
                .map(ToString::to_string)
                .map(Wrapper::from_inner),
            allow_failure: self.allow_failure,
        }
    }

    pub fn resolve(
        &self,
        global_settings: &GlobalSettings<
            impl Associations + AsRef<Path>,
            impl Associations + AsRef<Path>,
            impl Associations + AsRef<str>,
            impl Associations + AsRef<str>,
        >,
    ) -> OwnedMember {
        macro_rules! wrapper_to_owned {
            ($source:expr, $typename:ident) => {
                $source
                    .as_ref()
                    .to_string()
                    .pipe($typename::from_inner)
                    .pipe(Some)
            };
        }

        macro_rules! resolve_wrapper_option {
            ($field:ident, $typename:ident) => {
                match (&self.$field, &global_settings.$field) {
                    (Some(value), _) => wrapper_to_owned!(value, $typename),
                    (None, Some(value)) => wrapper_to_owned!(value, $typename),
                    (None, None) => None,
                }
            };
        }

        OwnedMember {
            directory: Wrapper::from_inner(if let Some(container) = &global_settings.container {
                container.as_ref().join(self.directory.as_ref())
            } else {
                self.directory.as_ref().to_path_buf()
            }),

            read_build_metadata: self
                .read_build_metadata
                .or(global_settings.read_build_metadata),

            install_missing_dependencies: self
                .install_missing_dependencies
                .or(global_settings.install_missing_dependencies),

            clean_before_build: self
                .clean_before_build
                .or(global_settings.clean_before_build),

            force_rebuild: self.force_rebuild.or(global_settings.force_rebuild),

            clean_after_build: self.clean_after_build.or(global_settings.clean_after_build),
            pacman: resolve_wrapper_option!(pacman, OwnedPacman),
            packager: resolve_wrapper_option!(packager, OwnedPackager),
            allow_failure: self.allow_failure.or(global_settings.allow_failure),
        }
    }
}
