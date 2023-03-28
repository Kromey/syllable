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

    pub fn with_nuclei<S: AsRef<str>>(mut self, nucleus: &[S]) -> Self {
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

            ..Default::default()
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

    probability_onset_exists: f64,
    probability_onset_is_cluster: f64,
    probability_nucleus_is_cluster: f64,
    probability_coda_exists: f64,
    probability_coda_is_cluster: f64,
}

impl Default for Syllable {
    fn default() -> Self {
        Self {
            onsets: Grapheme::map_slice(&[ "b", "c", "d", "f", "g", "h", "j", "k", "l", "m", "n", "p", "q", "r", "s", "t", "v", "w", "x", "y", "z" ]),
            onset_clusters: Grapheme::map_slice(&[ "ch", "sh", "bl", "cl", "fl", "pl", "gl", "br", "cr", "dr", "pr", "tr", "th", "sc", "sp", "st", "sl", "spr" ]),
            nucleus: Grapheme::map_slice(&[ "a", "e", "i", "o", "u" ]),
            nucleus_clusters: Grapheme::map_slice(&[ "ae", "ea", "ai", "ia", "au", "ay", "ie", "oi", "ou", "ey" ]),
            codas: Grapheme::map_slice(&[ "b", "c", "d", "f", "g", "h", "k", "l", "m", "n", "p", "r", "s", "t", "v", "x", "y"]),
            coda_clusters: Grapheme::map_slice(&[ "ck", "st", "sc", "ng", "nk", "rsh", "lsh", "rk", "rst", "nct", "xt" ]),

            probability_onset_exists: 0.95,
            probability_onset_is_cluster: 0.25,
            probability_nucleus_is_cluster: 0.25,
            probability_coda_exists: 0.10,
            probability_coda_is_cluster: 0.25,
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

        if rng.gen_bool(self.probability_onset_exists) {
            let onset = if self.onset_clusters.is_empty() || !rng.gen_bool(self.probability_onset_is_cluster) {
                self.onsets.choose(&mut rng).unwrap()
            } else {
                self.onset_clusters.choose(&mut rng).unwrap()
            };
            syllable.push_str(onset.as_ref());
        }

        // TODO: Should there be a probability that a nucleus exists?
        let nucleus = if self.nucleus_clusters.is_empty() || !rng.gen_bool(self.probability_nucleus_is_cluster) {
            self.nucleus.choose(&mut rng).unwrap()
        } else {
            self.nucleus_clusters.choose(&mut rng).unwrap()
        };
        syllable.push_str(nucleus.as_ref());

        if rng.gen_bool(self.probability_coda_exists) {
            let coda = if self.coda_clusters.is_empty() || !rng.gen_bool(self.probability_coda_is_cluster) {
                self.codas.choose(&mut rng).unwrap()
            } else {
                self.coda_clusters.choose(&mut rng).unwrap()
            };
            syllable.push_str(coda.as_ref());
        }

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
