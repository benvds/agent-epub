use crate::{CoverOptions, Error, ErrorKind, Result, valid_xml_text};
use image::{ImageFormat, ImageReader, Limits};
use std::{
    fs::File,
    io::{Cursor, Read},
    path::Path,
};

pub(crate) const MAX_BYTES: u64 = 10 * 1024 * 1024;
pub(crate) const MAX_SIDE: u32 = 8000;
pub(crate) const MAX_PIXELS: u64 = 16_000_000;

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
    if bytes.is_empty() || bytes.len() as u64 > MAX_BYTES {
        return Err(invalid("Cover must be nonempty and at most 10 MiB."));
    }
    let reader = || {
        ImageReader::new(Cursor::new(bytes))
            .with_guessed_format()
            .map_err(|e| invalid(format!("Cannot identify cover image: {e}")))
    };
    let format = reader()?.format();
    let types = match format {
        Some(ImageFormat::Png) => ("cover.png", "image/png"),
        Some(ImageFormat::Jpeg) => ("cover.jpg", "image/jpeg"),
        _ => {
            return Err(invalid(
                "Cover must contain JPEG or PNG image data; the filename alone is not sufficient.",
            ));
        }
    };
    let (width, height) = reader()?
        .into_dimensions()
        .map_err(|e| invalid(format!("Invalid cover image: {e}")))?;
    if width == 0
        || height == 0
        || width > MAX_SIDE
        || height > MAX_SIDE
        || u64::from(width) * u64::from(height) > MAX_PIXELS
    {
        return Err(invalid(
            "Cover dimensions must be positive, at most 8,000 pixels per side and 16 million pixels total.",
        ));
    }
    let mut reader = reader()?;
    let mut limits = Limits::default();
    limits.max_image_width = Some(MAX_SIDE);
    limits.max_image_height = Some(MAX_SIDE);
    limits.max_alloc = Some(128 * 1024 * 1024);
    reader.limits(limits);
    reader
        .decode()
        .map_err(|e| invalid(format!("Cover image cannot be decoded: {e}")))?;
    Ok(types)
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
