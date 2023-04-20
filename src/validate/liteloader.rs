use crate::validate::{SupportedGameVersions, ValidationError, ValidationResult};
use std::io::Cursor;
use zip::ZipArchive;

pub struct LiteLoaderValidator;

impl super::Validator for LiteLoaderValidator {
    fn get_file_extensions(&self) -> &[&str] {
        &["litemod", "jar"]
    }

    fn get_project_types(&self) -> &[&str] {
        &["mod"]
    }

    fn get_supported_loaders(&self) -> &[&str] {
        &["liteloader"]
    }

    fn get_supported_game_versions(&self) -> SupportedGameVersions {
        SupportedGameVersions::All
    }

    fn validate(
        &self,
        archive: &mut ZipArchive<Cursor<bytes::Bytes>>,
    ) -> Result<ValidationResult, ValidationError> {
        if archive.by_name("litemod.json").is_err() {
            return Ok(ValidationResult::Warning(
                "No litemod.json present for LiteLoader file.",
            ));
        }

        Ok(ValidationResult::Pass)
    }
}
