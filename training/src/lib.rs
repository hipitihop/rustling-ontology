extern crate rustling;
extern crate rustling_ontology_rules;
extern crate rustling_ontology_moment;
extern crate rustling_ontology_values;

use rustling_ontology_moment::*;
pub use rustling_ontology_values::dimension::*;
pub use rustling_ontology_values::output::*;
pub use rustling_ontology_values::output::ParsingContext;

macro_rules! example {
    ($v:expr, $check:expr, $($ex:expr),*) => {
        $( $v.push($crate::rustling::Example::new($ex, Box::new($check))); )*
    };
}

#[macro_use]
mod macros;
pub mod en;
pub mod es;
pub mod fr;

macro_rules! lang {
    ($lang:ident, $lang_test:ident, [$($example:ident),*]) => {
        pub fn $lang() -> Vec<::rustling::train::Example<Dimension>> {
            let mut v = vec![];
            $( $lang::$example(&mut v); )*
            v
        }
        #[cfg(test)]
        mod $lang_test {
            use rustling::*;
            use super::*;
            fn assert_examples(rules: &RuleSet<Dimension>, examples: Vec<Example<Dimension>>) {
                for ex in examples.iter() {
                    let stash = rules.apply_all(&ex.text.to_lowercase()).unwrap();
                    let correct_results = stash
                                .into_iter()
                                .filter(|candidate| candidate.root_node.range == Range(0, ex.text.len()) && ex.predicate.check(&candidate))
                                .collect::<Vec<_>>();
                    assert!(!correct_results.is_empty(), ex.text);
                }
            }
            #[test]
            fn test_examples() {
                let rules = ::rustling_ontology_rules::$lang().unwrap();
                let examples = $lang();
                assert_examples(&rules, examples);
            }
        }
    }
}

lang!(en, en_test, [examples_numbers, examples_time]);
lang!(fr, fr_test, [examples_numbers, examples_time]);
lang!(es, es_test, [examples_numbers, examples_time]);

