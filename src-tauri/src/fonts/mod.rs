use std::{collections::HashSet, path::PathBuf};

use fontdb::{Family, Style, Weight};

pub struct Fonts {
    db: fontdb::Database,
}

impl Fonts {
    pub fn new() -> Self {
        let mut db = fontdb::Database::new();
        db.load_system_fonts();

        Self { db }
    }

    pub fn all_families(&self) -> Vec<String> {
        let mut all_families = HashSet::new();

        for face in self.db.faces() {
            let families = &face.families;
            for (family, _) in families {
                all_families.insert(family.clone());
            }
        }

        all_families.into_iter().collect()
    }

    pub fn query_font_path(&self, family: &str, bold: bool, italic: bool) -> Option<PathBuf> {
        let font_id = self.db.query(&fontdb::Query {
            families: &[Family::Name(family)],
            weight: if bold { Weight::BOLD } else { Weight::NORMAL },
            stretch: fontdb::Stretch::Normal,
            style: if italic { Style::Italic } else { Style::Normal },
        })?;

        let face = self.db.face(font_id)?;
        let path = match &face.source {
            fontdb::Source::File(path_buf) => path_buf,
            fontdb::Source::SharedFile(path_buf, _) => path_buf,
            _ => return None,
        };

        Some(path.clone())
    }
}
