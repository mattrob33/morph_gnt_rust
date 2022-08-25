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

    pub fn get_plain_text_for_ref(bible_ref: &BibleRef) -> String {
        SblgntDatabase::get_plain_text_for_ref(&bible_ref)
    }

    pub fn get_plain_text_for_ref_range(range: &BibleRefRange) -> String {
        SblgntDatabase::get_plain_text_for_ref_range(&range)
    }

    pub fn get_gloss(lex: &String) -> String {
        SblgntDatabase::get_gloss_and_occ_for_lex(lex).0
    }

    pub fn get_occ(lex: &String) -> i64 {
        SblgntDatabase::get_gloss_and_occ_for_lex(lex).1
    }

    pub fn get_gloss_and_occ(lex: &String) -> (String, i64) {
        SblgntDatabase::get_gloss_and_occ_for_lex(lex)
    }
}