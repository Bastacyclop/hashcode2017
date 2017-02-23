use std::io;

pub struct Output {
    pub caches: Vec<Cache>,
}

impl Output {
    pub fn output<W: io::Write>(&self, w: &mut W) {
        writeln!(w, "{}", self.caches.len()).unwrap();
        for c in &self.caches {
            c.output(w);
        }
    }
}

pub struct Cache {
    pub id: u16,
    pub video_ids: Vec<u16>,
}

impl Cache {
    pub fn output<W: io::Write>(&self, w: &mut W) {
        write!(w, "{}", self.id).unwrap();
        for &vid in &self.video_ids {
            write!(w, " {}", vid).unwrap();
        }
        writeln!(w, "").unwrap();
    }
}
