use std::{fs::File, io::{Read, Seek, Write}, path::Path};
use walkdir::WalkDir;
use zip::{ZipWriter, unstable::write::FileOptionsExt, write::FileOptions};

pub fn compress_dir(src_dir: &Path, target: &Path) {
    let zipfile = File::create(target).unwrap();
    let dir = WalkDir::new(src_dir);
    zip_dir(
        &mut dir.into_iter().filter_map(|e| e.ok()),
        src_dir.to_str().unwrap(),
        zipfile,
    )
    .unwrap();
}

fn zip_dir<T>(
    it: &mut dyn Iterator<Item = walkdir::DirEntry>,
    prefix: &str,
    writer: T,
) -> zip::result::ZipResult<()>
where
    T: Write + Seek,
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::<'_, ()>::default()
        .compression_method(zip::CompressionMethod::Bzip2)
        .unix_permissions(0o755);
    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(Path::new(prefix)).unwrap();
        if path.is_file() {
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;
            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
            buffer.clear();
        } else if name.as_os_str().len() != 0 {
            zip.add_directory_from_path(name, options)?;
        }
    }
    zip.finish()?;
    Ok(())
}
