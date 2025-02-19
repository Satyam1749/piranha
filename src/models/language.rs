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

use getset::Getters;
use serde_derive::Deserialize;
use tree_sitter::{Parser, Query};

use crate::utilities::parse_toml;

use super::{
  default_configs::{default_language, GO, JAVA, KOTLIN, PYTHON, SWIFT, TSX, TYPESCRIPT},
  outgoing_edges::Edges,
  rule::Rules,
  scopes::{ScopeConfig, ScopeGenerator},
};

#[derive(Debug, Clone, Getters, PartialEq)]
pub struct PiranhaLanguage {
  #[get = "pub"]
  name: String,
  #[get = "pub"]
  supported_language: SupportedLanguage,
  #[get = "pub"]
  language: tree_sitter::Language,
  #[get = "pub(crate)"]
  rules: Option<Rules>,
  #[get = "pub(crate)"]
  edges: Option<Edges>,
  #[get = "pub(crate)"]
  scopes: Vec<ScopeGenerator>,
  #[get = "pub"]
  comment_nodes: Vec<String>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum SupportedLanguage {
  Java,
  Kotlin,
  Go,
  Swift,
  Ts,
  Tsx,
  Python,
}

impl Default for SupportedLanguage {
  fn default() -> Self {
    SupportedLanguage::Java
  }
}

impl PiranhaLanguage {
  pub fn is_comment(&self, kind: String) -> bool {
    self.comment_nodes().contains(&kind)
  }

  pub fn create_query(&self, query_str: String) -> Query {
    let query = Query::new(self.language, query_str.as_str());
    if let Ok(q) = query {
      return q;
    }
    panic!(
      "Could not parse the query : {:?} {:?}",
      query_str,
      query.err()
    );
  }

  pub fn parser(&self) -> Parser {
    let mut parser = Parser::new();
    parser
      .set_language(self.language)
      .expect("Could not set the language for the parser.");
    parser
  }

  #[cfg(test)]
  pub(crate) fn set_scopes(&mut self, scopes: Vec<ScopeGenerator>) {
    self.scopes = scopes;
  }
}

impl Default for PiranhaLanguage {
  fn default() -> Self {
    PiranhaLanguage::from(default_language().as_str())
  }
}

impl From<&str> for PiranhaLanguage {
  fn from(language: &str) -> Self {
    match language {
      JAVA => {
        let rules: Rules = parse_toml(include_str!("../cleanup_rules/java/rules.toml"));
        let edges: Edges = parse_toml(include_str!("../cleanup_rules/java/edges.toml"));
        Self {
          name: language.to_string(),
          supported_language: SupportedLanguage::Java,
          language: tree_sitter_java::language(),
          rules: Some(rules),
          edges: Some(edges),
          scopes: parse_toml::<ScopeConfig>(include_str!(
            "../cleanup_rules/java/scope_config.toml"
          ))
          .scopes()
          .to_vec(),
          comment_nodes: vec!["line_comment".to_string(), "block_comment".to_string()],
        }
      }
      GO => {
        let rules: Rules = parse_toml(include_str!("../cleanup_rules/go/rules.toml"));
        let edges: Edges = parse_toml(include_str!("../cleanup_rules/go/edges.toml"));
        PiranhaLanguage {
          name: language.to_string(),
          supported_language: SupportedLanguage::Go,
          language: tree_sitter_go::language(),
          rules: Some(rules),
          edges: Some(edges),
          scopes: parse_toml::<ScopeConfig>(include_str!("../cleanup_rules/go/scope_config.toml"))
            .scopes()
            .to_vec(),
          comment_nodes: vec!["comment".to_string()],
        }
      }
      KOTLIN => {
        let rules: Rules = parse_toml(include_str!("../cleanup_rules/kt/rules.toml"));
        let edges: Edges = parse_toml(include_str!("../cleanup_rules/kt/edges.toml"));
        PiranhaLanguage {
          name: language.to_string(),
          supported_language: SupportedLanguage::Kotlin,
          language: tree_sitter_kotlin::language(),
          rules: Some(rules),
          edges: Some(edges),
          scopes: parse_toml::<ScopeConfig>(include_str!("../cleanup_rules/kt/scope_config.toml"))
            .scopes()
            .to_vec(),
          comment_nodes: vec!["comment".to_string()],
        }
      }
      PYTHON => PiranhaLanguage {
        name: language.to_string(),
        supported_language: SupportedLanguage::Python,
        language: tree_sitter_python::language(),
        rules: None,
        edges: None,
        scopes: vec![],
        comment_nodes: vec![],
      },
      SWIFT => PiranhaLanguage {
        name: language.to_string(),
        supported_language: SupportedLanguage::Swift,
        language: tree_sitter_swift::language(),
        scopes: parse_toml::<ScopeConfig>(include_str!("../cleanup_rules/swift/scope_config.toml"))
          .scopes()
          .to_vec(),
        comment_nodes: vec!["comment".to_string(), "multiline_comment".to_string()],
        rules: None,
        edges: None,
      },
      TYPESCRIPT => PiranhaLanguage {
        name: language.to_string(),
        supported_language: SupportedLanguage::Ts,
        language: tree_sitter_typescript::language_typescript(),
        rules: None,
        edges: None,
        scopes: vec![],
        comment_nodes: vec![],
      },
      TSX => PiranhaLanguage {
        name: language.to_string(),
        supported_language: SupportedLanguage::Tsx,
        language: tree_sitter_typescript::language_tsx(),
        rules: None,
        edges: None,
        scopes: vec![],
        comment_nodes: vec![],
      },
      _ => panic!("Language not supported"),
    }
  }
}
