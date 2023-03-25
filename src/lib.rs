//! Syllable library

use rand::prelude::*;
use rand_xoshiro::Xoshiro256StarStar;

struct Grapheme(String, u32);

impl<S> From<S> for Grapheme
where
    S: Into<String>
{
    fn from(value: S) -> Self {
        Self(value.into(), 1)
    }
}

impl AsRef<str> for Grapheme {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Grapheme {
    fn new<S: Into<String>>(s: S, weight: u32) -> Self {
        Self(s.into(), weight)
    }

    fn string(&self) -> String {
        self.0.clone()
    }

    fn weight(&self) -> u32 {
        self.1
    }

    fn map_slice<S: AsRef<str>>(slice: &[S]) -> Vec<Self> {
        slice.iter().map(|onset| onset.as_ref().into()).collect()
    }
}

#[derive(Default)]
pub struct SyllableBuilder {
    onsets: Vec<Grapheme>,
    onset_clusters: Vec<Grapheme>,
    nucleus: Vec<Grapheme>,
    nucleus_clusters: Vec<Grapheme>,
    codas: Vec<Grapheme>,
    coda_clusters: Vec<Grapheme>,
}

impl SyllableBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_onsets<S: AsRef<str>>(mut self, onsets: &[S]) -> Self {
        self.onsets = Grapheme::map_slice(onsets);

        self
    }

    pub fn with_nucleus<S: AsRef<str>>(mut self, nucleus: &[S]) -> Self {
        self.nucleus = Grapheme::map_slice(nucleus);

        self
    }

    pub fn with_codas<S: AsRef<str>>(mut self, codas: &[S]) -> Self {
        self.codas = Grapheme::map_slice(codas);

        self
    }

    pub fn build(self) -> Syllable {
        self.into()
    }
}

impl From<SyllableBuilder> for Syllable {
    fn from(value: SyllableBuilder) -> Self {
        Self {
            onsets: value.onsets,
            onset_clusters: value.onset_clusters,
            nucleus: value.nucleus,
            nucleus_clusters: value.nucleus_clusters,
            codas: value.codas,
            coda_clusters: value.coda_clusters,
        }
    }
}

pub struct Syllable {
    onsets: Vec<Grapheme>,
    onset_clusters: Vec<Grapheme>,
    nucleus: Vec<Grapheme>,
    nucleus_clusters: Vec<Grapheme>,
    codas: Vec<Grapheme>,
    coda_clusters: Vec<Grapheme>,
}

impl Default for Syllable {
    fn default() -> Self {
        Self {
            onsets: vec![ "b", "c", "d", "f", "g", "h", "j", "k", "l", "m", "n", "p", "q", "r", "s", "t", "v", "w", "x", "y", "z" ].into_iter().map(|s| s.into()).collect(),
            onset_clusters: vec![], // vec![ "ch", "sh", "bl", "cl", "fl", "pl", "gl", "br", "cr", "dr", "pr", "tr", "th", "sc", "sp", "st", "sl", "spr" ].into_iter().map(|s| s.to_owned()).collect(),
            nucleus: vec![ "a", "e", "i", "o", "u" ].into_iter().map(|s| s.into()).collect(),
            nucleus_clusters: vec![], // vec![ "ae", "ea", "ai", "ia", "au", "ay", "ie", "oi", "ou", "ey" ].into_iter().map(|s| s.to_owned()).collect(),
            codas: vec![ "b", "c", "d", "f", "g", "h", "k", "l", "m", "n", "p", "r", "s", "t", "v", "x", "y"].into_iter().map(|s| s.into()).collect(),
            coda_clusters: vec![], // vec![ "ck", "st", "sc", "ng", "nk", "rsh", "lsh", "rk", "rst", "nct", "xt" ].into_iter().map(|s| s.to_owned()).collect(),
        }
    }
}

impl Syllable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn gen_syllable(&self) -> String {
        let mut rng = Xoshiro256StarStar::from_entropy();

        let mut syllable = String::new();

        let onset = if self.onset_clusters.is_empty() || rng.gen_bool(0.5) {
            self.onsets.choose(&mut rng).unwrap()
        } else {
            self.onset_clusters.choose(&mut rng).unwrap()
        };
        syllable.push_str(onset.as_ref());

        let nucleus = if self.nucleus_clusters.is_empty() || rng.gen_bool(0.5) {
            self.nucleus.choose(&mut rng).unwrap()
        } else {
            self.nucleus_clusters.choose(&mut rng).unwrap()
        };
        syllable.push_str(nucleus.as_ref());

        let coda = if self.coda_clusters.is_empty() || rng.gen_bool(0.5) {
            self.codas.choose(&mut rng).unwrap()
        } else {
            self.coda_clusters.choose(&mut rng).unwrap()
        };
        syllable.push_str(coda.as_ref());

        syllable
    }

    pub fn gen_name(&self, syllables: u32) -> String {
        let mut name = String::new();

        for _ in 0..syllables {
            name.push_str(&self.gen_syllable());
        }

        name
    }
}
