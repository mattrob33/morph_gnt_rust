use crate::word::Word;
use bible_ref::bible_ref::BibleRef;

pub struct Verse {
    pub bible_ref: BibleRef,
    pub words: Vec<Word>
}