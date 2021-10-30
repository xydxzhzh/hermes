/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use crate::ast::SourceRange;
use std::rc::Rc;
use support::NullTerminatedBuf;

/// An opaque value identifying a source registered with SourceManager.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct SourceId(pub u32);

impl SourceId {
    pub const INVALID: SourceId = SourceId(u32::MAX);

    pub fn is_valid(self) -> bool {
        self.0 != Self::INVALID.0
    }

    fn as_usize(self) -> usize {
        self.0 as usize
    }
}

/// SourceManager owns a collection of source buffers and their names and handles
/// reporting errors.
#[derive(Debug, Default)]
pub struct SourceManager {
    sources: Vec<(String, Rc<NullTerminatedBuf>)>,
}

impl SourceManager {
    pub fn new() -> SourceManager {
        Default::default()
    }

    /// Register a source buffer with its name.
    pub fn add_source<S: Into<String>>(&mut self, name: S, buf: NullTerminatedBuf) -> SourceId {
        assert!(
            self.sources.len() < SourceId::INVALID.0 as usize,
            "Too many sources",
        );
        self.sources.push((name.into(), Rc::new(buf)));
        SourceId(self.sources.len() as u32 - 1)
    }

    /// Obtain the name of a previously registered source buffer.
    pub fn source_name(&self, source_id: SourceId) -> &str {
        self.sources[source_id.as_usize()].0.as_str()
    }

    /// Obtain a reference to a previously registered source buffer.
    pub fn source_buffer(&self, source_id: SourceId) -> &NullTerminatedBuf {
        &self.sources[source_id.as_usize()].1
    }

    /// Obtain a Rc of a previously registered source buffer.
    pub fn source_buffer_rc(&self, source_id: SourceId) -> Rc<NullTerminatedBuf> {
        Rc::clone(&self.sources[source_id.as_usize()].1)
    }

    /// Report an error at the specified range in the specified source buffer.
    pub fn error<S: Into<String>>(&self, range: SourceRange, msg: S) {
        // NOTE: this method deliberately takes immutable `self`. A SourceManager
        // should be easily shareable. It will be implemented using interior
        // mutability.
        eprintln!(
            "{}:{}:{}: error: {}",
            self.source_name(range.file),
            range.start.line,
            range.start.col,
            msg.into()
        );
    }
    pub fn note<S: Into<String>>(&self, range: SourceRange, msg: S) {
        // NOTE: this method deliberately takes immutable `self`. A SourceManager
        // should be easily shareable. It will be implemented using interior
        // mutability.
        eprintln!(
            "{}:{}:{}: note: {}",
            self.source_name(range.file),
            range.start.line,
            range.start.col,
            msg.into()
        );
    }
    /// Report a warning at the specified range in the specified source buffer.
    pub fn warning<S: Into<String>>(&self, range: SourceRange, msg: S) {
        // NOTE: this method deliberately takes immutable `self`. A SourceManager
        // should be easily shareable. It will be implemented using interior
        // mutability.
        eprintln!(
            "{}:{}:{}: warning: {}",
            self.source_name(range.file),
            range.start.line,
            range.start.col,
            msg.into()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_test() {
        let mut sm = SourceManager::new();

        let id1 = sm.add_source("buf1", NullTerminatedBuf::from_str_copy("a"));
        let id2 = sm.add_source("buf2", NullTerminatedBuf::from_str_copy("bb"));

        assert_eq!("buf1", sm.source_name(id1));
        assert_eq!("buf2", sm.source_name(id2));

        assert_eq!(2, sm.source_buffer(id1).len());
        assert_eq!(3, sm.source_buffer(id2).len());

        let buf1 = sm.source_buffer_rc(id1);
        assert_eq!(2, buf1.len());
        assert_eq!(b"a\0", buf1.as_bytes());
    }
}
