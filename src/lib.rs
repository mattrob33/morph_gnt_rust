extern crate bible_ref;
extern crate sqlite;

pub mod verse;
pub mod word;
mod db;

use crate::db::SblgntDatabase;
use bible_ref::bible_ref::{BibleRef, BibleRefRange};
use bible_ref::book::Book::John;

pub struct GNT;

impl GNT {

    fn get_plain_text_for_ref(bible_ref: &BibleRef) -> String {
        SblgntDatabase::get_plain_text_for_ref(&bible_ref)
    }

    fn get_plain_text_for_ref_range(range: &BibleRefRange) -> String {
        SblgntDatabase::get_plain_text_for_ref_range(&range)
    }
}