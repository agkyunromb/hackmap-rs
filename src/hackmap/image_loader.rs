use super::common::*;
use super::config::ConfigRef;
use super::HackMap;
use anyhow::Result;
use std::collections::HashMap;
use std::io::Read;
use std::path::Path;
use D2Gfx::D2CellFileHeader;

pub struct DC6Buffer(Vec<u8>);
pub type DC6BufferRef = Rc<DC6Buffer>;

impl DC6Buffer {
    pub fn d2_cell_file_header(&self) -> &mut D2CellFileHeader {
        ptr_to_ref_mut(self.0.as_ptr() as *mut D2CellFileHeader).unwrap()
    }
}

impl std::ops::Deref for DC6Buffer {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for DC6Buffer {
    fn drop(&mut self) {
        D2CMP::CelFileFreeHardware(self.d2_cell_file_header());
    }
}

pub(super) struct ImageLoader {
    cfg: ConfigRef,
    cache: HashMap<String, DC6BufferRef>,
}

impl ImageLoader {
    pub fn new(cfg: ConfigRef) -> Self {
        Self {
            cfg,
            cache: HashMap::new(),
        }
    }

    pub fn init(&mut self) -> anyhow::Result<()> {
        D2ClientEx::Game::on_leave_game(|| {
            HackMap::image_loader().clear_cache();
        });

        Ok(())
    }

    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    pub fn load_image<T: AsRef<Path>>(&mut self, image_name: T) -> Option<DC6BufferRef> {
        if let Some(dc6) = self.load_image_from_cache(&image_name) {
            return Some(dc6);
        }

        let image_path = std::path::Path::new("hackmap").join(&image_name);

        let mut fs = std::fs::File::open(image_path).ok()?;
        let mut dc6 = vec![];

        fs.read_to_end(&mut dc6).ok()?;

        let ret = Rc::new(DC6Buffer(dc6));

        D2CMP::CelFileNormalize(ret.d2_cell_file_header(), -1);

        self.cache.insert(self.get_key(image_name), Rc::clone(&ret));

        Some(ret)
    }

    fn load_image_from_cache<T: AsRef<Path>>(&self, image_name: T) -> Option<DC6BufferRef> {
        let key = self.get_key(image_name);
        let buf = self.cache.get(&key);

        buf.map(Rc::clone)
    }

    fn get_key<T: AsRef<Path>>(&self, image_name: T) -> String {
        image_name.as_ref().to_string_lossy().into_owned()
    }
}
