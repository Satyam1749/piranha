/*
Copyright (c) 2022 Uber Technologies, Inc.

 <p>Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file
 except in compliance with the License. You may obtain a copy of the License at
 <p>http://www.apache.org/licenses/LICENSE-2.0

 <p>Unless required by applicable law or agreed to in writing, software distributed under the
 License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
 express or implied. See the License for the specific language governing permissions and
 limitations under the License.
*/

use std::collections::HashMap;

use getset::Getters;
use pyo3::prelude::{pyclass, pymethods};
use serde_derive::Serialize;

use crate::utilities::gen_py_str_methods;

#[derive(Serialize, Debug, Clone, Getters)]
#[pyclass]
pub(crate) struct Match {
  // Code snippet that matched
  #[get = "pub"]
  #[pyo3(get)]
  matched_string: String,
  // Range of the entire AST node captured by the match
  #[pyo3(get)]
  range: Range,
  // The mapping between tags and string representation of the AST captured.
  #[pyo3(get)]
  #[get = "pub"]
  matches: HashMap<String, String>,
}
gen_py_str_methods!(Match);

impl Match {
  pub(crate) fn new(
    matched_string: String, range: tree_sitter::Range, matches: HashMap<String, String>,
  ) -> Self {
    Self {
      matched_string,
      range: Range {
        start_byte: range.start_byte,
        end_byte: range.end_byte,
        start_point: Point {
          row: range.start_point.row,
          column: range.start_point.column,
        },
        end_point: Point {
          row: range.end_point.row,
          column: range.end_point.column,
        },
      },
      matches,
    }
  }

  /// Get the edit's replacement range.
  pub(crate) fn range(&self) -> tree_sitter::Range {
    tree_sitter::Range {
      start_byte: self.range.start_byte,
      end_byte: self.range.end_byte,
      start_point: tree_sitter::Point {
        row: self.range.start_point.row,
        column: self.range.start_point.column,
      },
      end_point: tree_sitter::Point {
        row: self.range.end_point.row,
        column: self.range.end_point.column,
      },
    }
  }
}
/// A range of positions in a multi-line text document, both in terms of bytes and of
/// rows and columns.
/// Note `LocalRange` derives serialize.
#[derive(serde_derive::Serialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[pyclass]
struct Range {
  #[pyo3(get)]
  start_byte: usize,
  #[pyo3(get)]
  end_byte: usize,
  #[pyo3(get)]
  start_point: Point,
  #[pyo3(get)]
  end_point: Point,
}
gen_py_str_methods!(Range);

/// A range of positions in a multi-line text document, both in terms of bytes and of
/// rows and columns.
#[derive(serde_derive::Serialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[pyclass]
struct Point {
  #[pyo3(get)]
  row: usize,
  #[pyo3(get)]
  column: usize,
}
gen_py_str_methods!(Point);
