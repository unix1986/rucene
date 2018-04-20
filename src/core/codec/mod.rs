mod per_field;
pub use self::per_field::*;

pub mod blocktree;
pub mod codec_util;
pub mod compressing;
pub mod format;
pub mod lucene50;
pub mod lucene53;
pub mod lucene54;
pub mod lucene60;
pub mod lucene62;
pub mod reader;
pub mod writer;

mod producer;
pub use self::producer::*;

use std::sync::{Arc, Mutex};
pub type DocValuesProducerRef = Arc<Mutex<Box<DocValuesProducer>>>;

use core::codec::format::PointsFormat;
use core::codec::format::{CompoundFormat, LiveDocsFormat, NormsFormat};
use core::codec::format::{DocValuesFormat, PostingsFormat, StoredFieldsFormat};
use core::codec::format::{FieldInfosFormat, SegmentInfoFormat, TermVectorsFormat};
use core::index::term::TermState;
use error::*;

#[derive(Clone)]
pub struct BlockTermState {
    /// Term ordinal, i.e. its position in the full list of
    /// sorted terms.
    ord: i64,
    /// how many docs have this term
    doc_freq: i32,

    /// total number of occurrences of this term
    total_term_freq: i64,

    /// the term's ord in the current block
    term_block_ord: i32,

    /// fp into the terms dict primary file (_X.tim) that holds this term
    // TODO: update BTR to nuke this
    block_file_pointer: i64,

    /// fields from IntBlockTermState
    doc_start_fp: i64,
    pos_start_fp: i64,
    pay_start_fp: i64,
    skip_offset: i64,
    last_pos_block_offset: i64,
    // docid when there is a single pulsed posting, otherwise -1
    // freq is always implicitly totalTermFreq in this case.
    singleton_doc_id: i32,
}

impl BlockTermState {
    fn new() -> BlockTermState {
        BlockTermState {
            ord: 0,
            doc_freq: 0,
            total_term_freq: 0,
            term_block_ord: 0,
            block_file_pointer: 0,

            doc_start_fp: 0,
            pos_start_fp: 0,
            pay_start_fp: 0,
            skip_offset: -1,
            last_pos_block_offset: -1,
            singleton_doc_id: -1,
        }
    }

    pub fn ord(&self) -> i64 {
        self.ord
    }

    pub fn doc_freq(&self) -> i32 {
        self.doc_freq
    }

    pub fn total_term_freq(&self) -> i64 {
        self.total_term_freq
    }

    pub fn term_block_ord(&self) -> i32 {
        self.term_block_ord
    }

    pub fn block_file_pointer(&self) -> i64 {
        self.block_file_pointer
    }

    pub fn doc_start_fp(&self) -> i64 {
        self.doc_start_fp
    }
    pub fn pos_start_fp(&self) -> i64 {
        self.pos_start_fp
    }
    pub fn pay_start_fp(&self) -> i64 {
        self.pay_start_fp
    }
    pub fn skip_offset(&self) -> i64 {
        self.skip_offset
    }
    pub fn last_pos_block_offset(&self) -> i64 {
        self.last_pos_block_offset
    }
    pub fn singleton_doc_id(&self) -> i32 {
        self.singleton_doc_id
    }
}

impl TermState for BlockTermState {}

pub fn check_ascii_with_limit(s: &str, limit: usize) -> Result<()> {
    if s.chars().count() != s.len() || s.len() > limit {
        bail!(
            "Non ASCII or longer than {} characters in length [got {}]",
            limit,
            s
        )
    } else {
        Ok(())
    }
}

pub trait Codec: Send + Sync {
    fn name(&self) -> &str;
    fn postings_format(&self) -> &PostingsFormat;
    fn doc_values_format(&self) -> &DocValuesFormat;
    fn stored_fields_format(&self) -> &StoredFieldsFormat;
    fn term_vectors_format(&self) -> &TermVectorsFormat;
    fn field_infos_format(&self) -> &FieldInfosFormat;
    fn segment_info_format(&self) -> &SegmentInfoFormat;
    fn norms_format(&self) -> &NormsFormat;
    fn live_docs_format(&self) -> &LiveDocsFormat;
    fn compound_format(&self) -> &CompoundFormat;

    /// Encodes/decodes points index
    fn points_format(&self) -> &PointsFormat;
}

pub fn codec_for_name(name: &str) -> Result<Box<Codec>> {
    match name {
        "Lucene62" => Ok(Box::new(lucene62::Lucene62Codec::default())),
        _ => bail!("Invalid codec name: {}", name),
    }
}