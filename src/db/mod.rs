use bible_ref::bible_ref::{BibleRef, BibleRefRange};
use bible_ref::book::Book;
use crate::verse::Verse;
use crate::word::Word;
use sqlite::State;
use std::convert::TryInto;

pub struct InvalidRangeError;

pub struct SblgntDatabase;

impl SblgntDatabase {

    pub fn get_plain_text_for_ref(bible_ref: &BibleRef) -> String {
        let connection = sqlite::open("src/db/gnt.db").unwrap();

        let book = book_to_nt_int(&bible_ref.book);

        let mut statement = connection
            .prepare("SELECT group_concat(word, ' ') FROM words WHERE book=? AND chapter=? AND verse=? ORDER BY book, chapter, verse, word_num").unwrap()
            .bind(1, book).unwrap()
            .bind(2, bible_ref.chapter as i64).unwrap()
            .bind(3, bible_ref.verse as i64).unwrap();

        statement.next();
        statement.read::<String>(0).unwrap()
    }

    pub fn get_plain_text_for_ref_range(range: &BibleRefRange) -> String {
        let connection = sqlite::open("src/db/gnt.db").unwrap();

        let start_book = book_to_nt_int(&range.start.book);
        let end_book = book_to_nt_int(&range.end.book);

        let start_id = format!("{:02}{:02}{:02}", start_book, range.start.chapter, range.start.verse);
        let end_id = format!("{:02}{:02}{:02}", end_book, range.end.chapter, range.end.verse);

        let mut statement = connection
            .prepare(
                "
                SELECT group_concat(word, ' ')
                FROM words
                WHERE
                    printf('%02d%02d%02d', book, chapter, verse)  >= ?
                    AND
                    printf('%02d%02d%02d', book, chapter, verse)  <= ?
                ORDER BY book,chapter,verse,word_num
                "
            ).unwrap()
            .bind(1, start_id.as_str()).unwrap()
            .bind(2, end_id.as_str()).unwrap();

        statement.next();
        statement.read::<String>(0).unwrap()
    }

    pub fn get_gloss_and_occ_for_lex(lex: &str) -> (String, i64) {
        let connection = sqlite::open("src/db/gnt.db").unwrap();

        let mut statement = connection
            .prepare("SELECT gloss, occ FROM glosses WHERE lex = ?").unwrap()
            .bind(1, lex).unwrap();

        statement.next();

        let gloss = match statement.read::<String>(0) {
          Ok(gloss) => gloss,
            Err(_) => String::from("")
        };

        let occ = match statement.read::<i64>(1) {
          Ok(occ) => occ,
            Err(_) => 0
        };

        (gloss, occ)
    }

    pub fn get_distinct_logoi_candidates() -> Vec<(String, String)> {
        let connection = sqlite::open("src/db/gnt.db").unwrap();

        let mut statement = connection
            .prepare("SELECT DISTINCT word, lex FROM words WHERE length(word) == 5 OR length(word) == 6").unwrap();

        let mut words: Vec<(String, String)> = vec![];

        while let State::Row = statement.next().unwrap() {
            let word = statement.read::<String>(0).unwrap();
            let lex = statement.read::<String>(1).unwrap();
            words.push((word, lex));
        }

        words
    }
}

fn book_to_nt_int(book: &Book) -> i64 {
    let raw_book = match book {
        Book::Matthew => 40,
        Book::Mark => 41,
        Book::Luke => 42,
        Book::John => 43,
        Book::Acts => 44,
        Book::Romans => 45,
        Book::FirstCorinthians => 46,
        Book::SecondCorinthians => 47,
        Book::Galatians => 48,
        Book::Ephesians => 49,
        Book::Philippians => 50,
        Book::Colossians => 51,
        Book::FirstThessalonians => 52,
        Book::SecondThessalonians => 53,
        Book::FirstTimothy => 54,
        Book::SecondTimothy => 55,
        Book::Titus => 56,
        Book::Philemon => 57,
        Book::Hebrews => 58,
        Book::James => 59,
        Book::FirstPeter => 60,
        Book::SecondPeter => 61,
        Book::FirstJohn => 62,
        Book::SecondJohn => 63,
        Book::ThirdJohn => 64,
        Book::Jude => 65,
        Book::Revelation => 66,
        _ => panic!("Invalid book")
    };
    return raw_book - 40;
}