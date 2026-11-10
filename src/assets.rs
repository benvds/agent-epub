use crate::{Error, ErrorKind, Result};
use image::{ImageFormat, ImageReader, Limits, codecs::png::PngDecoder};
use percent_encoding::percent_decode_str;
use std::{
    collections::HashMap,
    fs::File,
    io::{Cursor, Read},
    path::{Path, PathBuf},
};

pub(crate) const MAX_BYTES: u64 = 10 * 1024 * 1024;
pub(crate) const MAX_SIDE: u32 = 8000;
pub(crate) const MAX_PIXELS: u64 = 16_000_000;
pub(crate) const MAX_IMAGES: usize = 128;
pub(crate) const MAX_TOTAL_BYTES: usize = 64 * 1024 * 1024;

fn invalid(message: impl Into<String>) -> Error {
    Error::new(ErrorKind::Input, message).issue("invalid_image", "Supply a valid static JPEG/PNG within 10 MiB, 8,000 pixels per side and 16 million pixels.")
}

fn limits() -> Limits {
    let mut limits = Limits::default();
    limits.max_image_width = Some(MAX_SIDE);
    limits.max_image_height = Some(MAX_SIDE);
    limits.max_alloc = Some(128 * 1024 * 1024);
    limits
}

pub(crate) fn inspect(bytes: &[u8]) -> Result<(&'static str, &'static str)> {
    if bytes.is_empty() || bytes.len() as u64 > MAX_BYTES {
        return Err(invalid("Image must be nonempty and at most 10 MiB."));
    }
    let reader = || {
        let mut reader = ImageReader::new(Cursor::new(bytes))
            .with_guessed_format()
            .map_err(|e| invalid(format!("Cannot identify image: {e}")))?;
        reader.limits(limits());
        Ok::<_, Error>(reader)
    };
    let types = match reader()?.format() {
        Some(ImageFormat::Png) => ("png", "image/png"),
        Some(ImageFormat::Jpeg) => ("jpg", "image/jpeg"),
        _ => {
            return Err(invalid(
                "Image must contain JPEG or PNG data; its filename is not sufficient.",
            ));
        }
    };
    let (width, height) = reader()?
        .into_dimensions()
        .map_err(|e| invalid(format!("Invalid image dimensions: {e}")))?;
    if width == 0
        || height == 0
        || width > MAX_SIDE
        || height > MAX_SIDE
        || u64::from(width) * u64::from(height) > MAX_PIXELS
    {
        return Err(invalid(
            "Image dimensions must be positive, at most 8,000 pixels per side and 16 million pixels total.",
        ));
    }
    if types.0 == "png"
        && PngDecoder::with_limits(Cursor::new(bytes), limits())
            .and_then(|decoder| decoder.is_apng())
            .map_err(|e| invalid(format!("Invalid PNG: {e}")))?
    {
        return Err(invalid(
            "Animated PNG images are unsupported; supply a static image.",
        ));
    }
    reader()?
        .decode()
        .map_err(|e| invalid(format!("Image cannot be decoded: {e}")))?;
    Ok(types)
}

pub(crate) fn validate_packaged(bytes: &[u8], media_type: &str, path: &Path) -> Result<()> {
    let (_, actual) = inspect(bytes)
        .map_err(|e| Error::new(ErrorKind::Validation, format!("{}: {e}", path.display())))?;
    if actual != media_type {
        return Err(Error::new(
            ErrorKind::Validation,
            format!(
                "{}: image media type does not match its data.",
                path.display()
            ),
        ));
    }
    Ok(())
}

#[derive(Debug)]
pub(crate) struct Image {
    pub source: PathBuf,
    pub filename: String,
    pub media_type: &'static str,
    pub bytes: Vec<u8>,
}

#[derive(Default)]
pub(crate) struct Assets {
    pub images: Vec<Image>,
    lookup: HashMap<PathBuf, usize>,
    total: usize,
}

impl Assets {
    pub fn load(&mut self, destination: &str, base: Option<&Path>) -> Result<String> {
        let unsupported = || {
            Error::new(ErrorKind::Input, format!("unsupported image source {destination:?}."))
            .issue("unsupported_image_source", "Use a source-relative static JPEG/PNG file path with forward slashes and no URL, query or fragment.")
        };
        let path = percent_decode_str(destination)
            .decode_utf8()
            .map_err(|_| unsupported())?;
        if path.is_empty()
            || Path::new(path.as_ref()).is_absolute()
            || path.contains([':', '\\', '?', '#', '\0'])
        {
            return Err(unsupported());
        }
        let base = base.ok_or_else(|| Error::new(ErrorKind::Input, "Inline streamed images require an explicit asset base.")
            .issue("image_base_required", "Pass --base-dir pointing to the directory containing the inline manuscript's image paths."))?;
        let source = base.join(path.as_ref()).canonicalize().map_err(|e| {
            Error::new(ErrorKind::Io, format!("Image {destination:?}: {e}"))
                .issue("image_not_found", "Supply a readable image at the source-relative path, or correct the path/base directory.")
        })?;
        if let Some(&index) = self.lookup.get(&source) {
            return Ok(self.images[index].filename.clone());
        }
        if self.images.len() >= MAX_IMAGES {
            return Err(
                invalid("At most 128 unique inline image files are supported.").issue(
                    "image_limit_exceeded",
                    "Use at most 128 unique images and 64 MiB total inline image data.",
                ),
            );
        }
        if !std::fs::metadata(&source)
            .map_err(|e| Error::new(ErrorKind::Io, e.to_string()))?
            .is_file()
        {
            return Err(invalid("Image paths must identify regular files."));
        }
        let mut bytes = Vec::new();
        let file = File::open(&source).map_err(|e| {
            Error::new(ErrorKind::Io, format!("Image {destination:?}: {e}")).issue(
                "image_not_found",
                "Check that the image exists and is readable.",
            )
        })?;
        if !file
            .metadata()
            .map_err(|e| Error::new(ErrorKind::Io, e.to_string()))?
            .is_file()
        {
            return Err(invalid("Image paths must identify regular files."));
        }
        file.take(MAX_BYTES + 1)
            .read_to_end(&mut bytes)
            .map_err(|e| {
                Error::new(ErrorKind::Io, format!("Image {destination:?}: {e}")).issue(
                    "image_not_found",
                    "Check the image file and read permissions.",
                )
            })?;
        if self.total + bytes.len() > MAX_TOTAL_BYTES {
            return Err(
                invalid("Inline images exceed the 64 MiB total limit.").issue(
                    "image_limit_exceeded",
                    "Use at most 128 unique images and 64 MiB total inline image data.",
                ),
            );
        }
        let (extension, media_type) = inspect(&bytes)?;
        let filename = format!("images/image-{:04}.{extension}", self.images.len() + 1);
        self.total += bytes.len();
        self.lookup.insert(source.clone(), self.images.len());
        self.images.push(Image {
            source,
            filename: filename.clone(),
            media_type,
            bytes,
        });
        Ok(filename)
    }
}
