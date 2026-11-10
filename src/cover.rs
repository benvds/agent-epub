use crate::assets::MAX_BYTES;
use crate::{CoverOptions, Error, ErrorKind, Result, valid_xml_text};
use std::{fs::File, io::Read, path::Path};

#[derive(Debug)]
pub(crate) struct Cover {
    pub bytes: Vec<u8>,
    pub filename: &'static str,
    pub media_type: &'static str,
    pub alt: String,
}

fn invalid(message: impl Into<String>) -> Error {
    Error::new(ErrorKind::Input, message)
}

pub(crate) fn check_image(bytes: &[u8]) -> Result<(&'static str, &'static str)> {
    let (extension, media_type) = crate::assets::inspect(bytes)?;
    Ok((
        if extension == "png" {
            "cover.png"
        } else {
            "cover.jpg"
        },
        media_type,
    ))
}

pub(crate) fn load(options: &CoverOptions, title: &str) -> Result<Cover> {
    if options.path.as_os_str().is_empty() {
        return Err(invalid("Supply a cover image path."));
    }
    let mut bytes = Vec::new();
    File::open(&options.path)
        .and_then(|f| f.take(MAX_BYTES + 1).read_to_end(&mut bytes))
        .map_err(|e| {
            Error::new(
                ErrorKind::Io,
                format!("Cover {}: {e}", options.path.display()),
            )
        })?;
    let (filename, media_type) = check_image(&bytes)
        .map_err(|e| invalid(format!("Cover {}: {e}", options.path.display())))?;
    let alt = options
        .alt
        .clone()
        .unwrap_or_else(|| format!("Cover of {title}"));
    if alt.trim().is_empty() || !valid_xml_text(&alt) {
        return Err(invalid(
            "Cover alternative text must be nonempty and contain valid XML characters.",
        ));
    }
    Ok(Cover {
        bytes,
        filename,
        media_type,
        alt,
    })
}

pub(crate) fn validate_packaged(bytes: &[u8], media_type: &str, path: &Path) -> Result<()> {
    let (_, actual) = check_image(bytes)
        .map_err(|e| Error::new(ErrorKind::Validation, format!("{}: {e}", path.display())))?;
    if actual != media_type {
        return Err(Error::new(
            ErrorKind::Validation,
            format!(
                "{}: cover media type does not match its image data.",
                path.display()
            ),
        ));
    }
    Ok(())
}
