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
use crate::models::{default_configs::JAVA, language::PiranhaLanguage};
use std::collections::HashSet;

use super::InstantiatedRule;
use {
  super::Rule,
  crate::models::{
    constraint::Constraint, rule_store::RuleStore, source_code_unit::SourceCodeUnit,
  },
  std::collections::HashMap,
  std::path::PathBuf,
};

/// Tests whether a valid rule can be correctly instantiated given valid substitutions.
#[test]
fn test_rule_try_instantiate_positive() {
  let holes = HashSet::from([String::from("variable_name")]);
  let rule = Rule::new("test","(((assignment_expression left: (_) @a.lhs right: (_) @a.rhs) @abc) (#eq? @a.lhs \"@variable_name\"))",
        "@abc", "",holes, HashSet::new());
  let substitutions: HashMap<String, String> = HashMap::from([
    (String::from("variable_name"), String::from("foobar")),
    (String::from("@a.lhs"), String::from("something")), // Should not substitute, since it `a.lhs` is not in `rule.holes`
  ]);
  let instantiated_rule = InstantiatedRule::new(&rule, &substitutions);
  assert_eq!(
    instantiated_rule.query(),
    "(((assignment_expression left: (_) @a.lhs right: (_) @a.rhs) @abc) (#eq? @a.lhs \"foobar\"))"
  )
}

/// Tests whether a valid rule is *not* instantiated given invalid substitutions.
#[test]
#[should_panic]
fn test_rule_try_instantiate_negative() {
  let rule = Rule::new("test","(((assignment_expression left: (_) @a.lhs right: (_) @a.rhs) @abc) (#eq? @a.lhs \"@variable_name\"))",
        "abc", "",HashSet::from([String::from("variable_name")]), HashSet::new());
  let substitutions: HashMap<String, String> = HashMap::from([
    (String::from("@a.lhs"), String::from("something")), // Should not substitute, since it `a.lhs` is not in `rule.holes`
  ]);
  let _ = InstantiatedRule::new(&rule, &substitutions);
}

/// Positive tests for `rule.get_edit` method for given rule and input source code.
#[test]
fn test_get_edit_positive_recursive() {
  let _rule = Rule::new("test", "(
                           ((local_variable_declaration
                                           declarator: (variable_declarator
                                                               name: (_) @variable_name
                                                               value: [(true) (false)] @init)) @variable_declaration)
                           )", "variable_declaration", "" ,HashSet::new(), 
                           HashSet::from([
                          Constraint::new(String::from("(method_declaration) @md"),
                            vec![String::from("(
                              ((assignment_expression
                                              left: (_) @a.lhs
                                              right: (_) @a.rhs) @assignment)
                              (#eq? @a.lhs \"@variable_name\")
                              (#not-eq? @a.rhs \"@init\")
                            )")]),
                          ]));
  let rule = InstantiatedRule::new(&_rule, &HashMap::new());
  let source_code = "class Test {
          pub void foobar(){
            boolean isFlagTreated = true;
            isFlagTreated = true;
            if (isFlagTreated) {
              // Do something;
            }
          }
        }";

  let mut rule_store = RuleStore::default();

  let mut parser = PiranhaLanguage::from(JAVA).parser();

  let source_code_unit = SourceCodeUnit::new(
    &mut parser,
    source_code.to_string(),
    &HashMap::new(),
    PathBuf::new().as_path(),
    rule_store.piranha_args(),
  );
  let node = source_code_unit.root_node();
  let matches = source_code_unit.get_matches(&rule, &mut rule_store, node, true);
  assert!(!matches.is_empty());

  let edit = source_code_unit.get_edit(&rule, &mut rule_store, node, true);
  assert!(edit.is_some());
}

/// Negative tests for `rule.get_edit` method for given rule and input source code.
#[test]
fn test_get_edit_negative_recursive() {
  let _rule = Rule::new("test", "(
                           ((local_variable_declaration
                                           declarator: (variable_declarator
                                                               name: (_) @variable_name
                                                               value: [(true) (false)] @init)) @variable_declaration)
                           )", "variable_declaration", "" ,HashSet::new(), 
                           HashSet::from([
                          Constraint::new(String::from("(method_declaration) @md"),
                            vec![String::from("(
                              ((assignment_expression
                                              left: (_) @a.lhs
                                              right: (_) @a.rhs) @assignment)
                              (#eq? @a.lhs \"@variable_name\")
                              (#not-eq? @a.rhs \"@init\")
                            )")]),
                          ]));
  let source_code = "class Test {
          pub void foobar(){
            boolean isFlagTreated = true;
            isFlagTreated = false;
            if (isFlagTreated) {
              // Do something;
            }
          }
        }";

  let rule = InstantiatedRule::new(&_rule, &HashMap::new());
  let mut rule_store = RuleStore::default();
  let mut parser = PiranhaLanguage::from(JAVA).parser();

  let source_code_unit = SourceCodeUnit::new(
    &mut parser,
    source_code.to_string(),
    &HashMap::new(),
    PathBuf::new().as_path(),
    rule_store.piranha_args(),
  );
  let node = source_code_unit.root_node();
  let matches = source_code_unit.get_matches(&rule, &mut rule_store, node, true);
  assert!(matches.is_empty());
  let edit = source_code_unit.get_edit(&rule, &mut rule_store, node, true);
  assert!(edit.is_none());
}

/// Positive tests for `rule.get_edit_for_context` method for given rule and input source code.
#[test]
fn test_get_edit_for_context_positive() {
  let _rule = Rule::new(
    "test",
    "(
          (binary_expression
              left : (_)* @lhs
              operator:\"&&\"
              right: [(true) (parenthesized_expression (true))]
          )
      @binary_expression)",
    "binary_expression",
    "",
    HashSet::new(),
    HashSet::new(),
  );
  let rule = InstantiatedRule::new(&_rule, &HashMap::new());

  let source_code = "class A {
          boolean f = something && true;
        }";

  let mut rule_store = RuleStore::default();

  let mut parser = PiranhaLanguage::from(JAVA).parser();

  let source_code_unit = SourceCodeUnit::new(
    &mut parser,
    source_code.to_string(),
    &HashMap::new(),
    PathBuf::new().as_path(),
    rule_store.piranha_args(),
  );
  let edit =
    source_code_unit.get_edit_for_context(41_usize, 44_usize, &mut rule_store, &vec![rule]);
  // let edit = rule.get_edit(&source_code_unit, &mut rule_store, node, true);
  assert!(edit.is_some());
}

/// Negative tests for `rule.get_edit_for_context` method for given rule and input source code.
#[test]
fn test_get_edit_for_context_negative() {
  let _rule = Rule::new(
    "test",
    "(
          (binary_expression
              left : (_)* @lhs
              operator:\"&&\"
              right: [(true) (parenthesized_expression (true))]
          )
      @binary_expression)",
    "binary_expression",
    "",
    HashSet::new(),
    HashSet::new(),
  );
  let rule = InstantiatedRule::new(&_rule, &HashMap::new());
  let source_code = "class A {
          boolean f = true;
          boolean x = something && true;
        }";

  let mut rule_store = RuleStore::default();

  let mut parser = PiranhaLanguage::from(JAVA).parser();

  let source_code_unit = SourceCodeUnit::new(
    &mut parser,
    source_code.to_string(),
    &HashMap::new(),
    PathBuf::new().as_path(),
    rule_store.piranha_args(),
  );
  let edit =
    source_code_unit.get_edit_for_context(29_usize, 33_usize, &mut rule_store, &vec![rule]);
  // let edit = rule.get_edit(&source_code_unit, &mut rule_store, node, true);
  assert!(edit.is_none());
}
