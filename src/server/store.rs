use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

const DEFAULT_ROOT_FOLDER_NAME: &str = "ggnetwork";

struct PathKey {
    path_name: String,
    filename: String,
}

impl PathKey {
    fn first_path_name(&self) -> &str {
        self.path_name.split('/').next().unwrap_or_default()
    }

    fn full_path(&self) -> String {
        format!("{}/{}", self.path_name, self.filename)
    }
}

type PathTransformFunc = fn(&str) -> PathKey;

fn default_path_transform_func(key: &str) -> PathKey {
    PathKey {
        path_name: key.to_string(),
        filename: key.to_string(),
    }
}

struct StoreOpts {
    root: String,
    path_transform_func: PathTransformFunc,
}

struct Store {
    opts: StoreOpts,
}

impl Store {
    fn new(opts: StoreOpts) -> Self {
        let path_transform_func = opts.path_transform_func;
        let root = if opts.root.is_empty() {
            DEFAULT_ROOT_FOLDER_NAME.to_string()
        } else {
            opts.root
        };

        Store {
            opts: StoreOpts {
                root,
                path_transform_func,
            },
        }
    }

    fn has(&self, id: &str, key: &str) -> bool {
        let path_key = (self.opts.path_transform_func)(key);
        let full_path_with_root = format!("{}/{}/{}", self.opts.root, id, path_key.full_path());

        fs::metadata(&full_path_with_root).is_ok()
    }

    fn clear(&self) -> io::Result<()> {
        fs::remove_dir_all(&self.opts.root)
    }

    fn delete(&self, id: &str, key: &str) -> io::Result<()> {
        let path_key = (self.opts.path_transform_func)(key);
        let first_path_name_with_root =
            format!("{}/{}/{}", self.opts.root, id, path_key.first_path_name());

        fs::remove_dir_all(first_path_name_with_root)?;
        Ok(())
    }

    fn write(&self, id: &str, key: &str, mut reader: impl Read) -> io::Result<u64> {
        let (path_key, mut file) = self.open_file_for_writing(id, key)?;

        let result = io::copy(&mut reader, &mut file)?;

        Ok(result)
    }

    fn open_file_for_writing(&self, id: &str, key: &str) -> io::Result<(PathKey, fs::File)> {
        let path_key = (self.opts.path_transform_func)(key);
        let path_name_with_root = format!("{}/{}/{}", self.opts.root, id, path_key.path_name);

        fs::create_dir_all(&path_name_with_root)?;
        let full_path_with_root = format!("{}/{}/{}", self.opts.root, id, path_key.full_path());

        Ok((path_key, fs::File::create(full_path_with_root)?))
    }

    fn read(&self, id: &str, key: &str) -> io::Result<(u64, Box<dyn Read + 'static>)> {
        let (path_key, file) = self.read_stream(id, key)?;

        Ok((
            path_key.filename.parse().unwrap_or_default(),
            Box::new(file),
        ))
    }

    fn read_stream(&self, id: &str, key: &str) -> io::Result<(PathKey, fs::File)> {
        let path_key = (self.opts.path_transform_func)(key);
        let full_path_with_root = format!("{}/{}/{}", self.opts.root, id, path_key.full_path());

        Ok((path_key, fs::File::open(full_path_with_root)?))
    }
}
