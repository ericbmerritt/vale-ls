use std::borrow::Cow;

use tower_lsp::lsp_types::*;
use yaml_rust::YamlLoader;

use crate::error::Error;

pub enum Extends {
    Existence,
    Substitution,
    Occurrence,
    Repetition,
    Consistency,
    Conditional,
    Capitalization,
    Metric,
    Spelling,
    Sequence,
    Script,
    Invalid,
}

pub struct Rule {
    pub extends: Extends,
    pub source: String,
}

fn vec_to_completions(vec: Vec<&str>) -> Vec<CompletionItem> {
    vec.into_iter()
        .map(|s| CompletionItem {
            label: s.to_string(),
            kind: Some(CompletionItemKind::VALUE),
            ..CompletionItem::default()
        })
        .collect()
}

impl Rule {
    pub(crate) fn new(rule_path: &str) -> Result<Rule, Error> {
        let src = std::fs::read_to_string(rule_path)?;
        match YamlLoader::load_from_str(&src) {
            Ok(docs) => {
                if docs.len() < 1 {
                    return Ok(Rule {
                        extends: Extends::Invalid,
                        source: "".to_string(),
                    });
                }
                let doc = docs[0].clone();
                let extends = match doc["extends"].as_str().unwrap_or("invalid") {
                    "existence" => Extends::Existence,
                    "substitution" => Extends::Substitution,
                    "occurrence" => Extends::Occurrence,
                    "repetition" => Extends::Repetition,
                    "consistency" => Extends::Consistency,
                    "conditional" => Extends::Conditional,
                    "capitalization" => Extends::Capitalization,
                    "metric" => Extends::Metric,
                    "spelling" => Extends::Spelling,
                    "sequence" => Extends::Sequence,
                    "script" => Extends::Script,
                    _ => Extends::Invalid,
                };
                Ok(Rule {
                    extends,
                    source: doc["link"].as_str().unwrap_or("").to_string(),
                })
            }
            Err(_) => Ok(Rule {
                extends: Extends::Invalid,
                source: "".to_string(),
            }),
        }
    }

    pub(crate) fn source(&self) -> String {
        self.source.clone()
    }

    pub(crate) fn complete(&self, line: &str) -> Result<Vec<CompletionItem>, Error> {
        let mut completions = Vec::new();

        if line.contains("extends:") {
            completions = vec_to_completions(vec![
                "existence",
                "substitution",
                "occurrence",
                "repetition",
                "consistency",
                "conditional",
                "capitalization",
                "metric",
                "spelling",
                "sequence",
                "script",
            ]);
        } else if line.contains("level:") {
            completions = vec_to_completions(vec!["suggestion", "warning", "error"]);
        }

        Ok(completions)
    }

    /// Returns the documentation for a given token, if it exists.
    pub(crate) fn token_info(&self, token: &str) -> Option<Cow<'static, str>> {
        let tok = token.trim_end_matches(':');
        match self.extends {
            Extends::Existence => self.existence(tok),
            Extends::Substitution => self.substitution(tok),
            Extends::Occurrence => self.occurrence(tok),
            Extends::Repetition => self.repetition(tok),
            Extends::Consistency => self.consistency(tok),
            Extends::Conditional => self.conditional(tok),
            Extends::Capitalization => self.capitalization(tok),
            Extends::Metric => self.metric(tok),
            Extends::Spelling => self.spelling(tok),
            Extends::Sequence => self.sequence(tok),
            Extends::Script => self.script(tok),
            Extends::Invalid => None,
        }
    }

    fn common(&self, token: &str, example: &str) -> Option<Cow<'static, str>> {
        match token {
            "extends" => {
                let docs = include_str!("../doc/yml/extends.md");
                let info = format!("{}\n\n## Example\n\n{}", docs, example);
                Some(info.into())
            }
            "message" => Some(include_str!("../doc/yml/message.md").into()),
            "level" => Some(include_str!("../doc/yml/level.md").into()),
            "scope" => Some(include_str!("../doc/yml/scope.md").into()),
            "link" => Some(include_str!("../doc/yml/link.md").into()),
            "limit" => Some(include_str!("../doc/yml/limit.md").into()),
            "action" => Some(include_str!("../doc/yml/action.md").into()),
            _ => None,
        }
    }

    fn existence(&self, key: &str) -> Option<Cow<'static, str>> {
        let example = include_str!("../doc/yml/existence/example.md");
        match key {
            "append" => Some(include_str!("../doc/yml/existence/append.md").into()),
            "ignorecase" => Some(include_str!("../doc/yml/existence/ignorecase.md").into()),
            "nonword" => Some(include_str!("../doc/yml/existence/nonword.md").into()),
            "raw" => Some(include_str!("../doc/yml/existence/raw.md").into()),
            "tokens" => Some(include_str!("../doc/yml/existence/tokens.md").into()),
            "exceptions" => Some(include_str!("../doc/yml/existence/exceptions.md").into()),
            _ => self.common(key, example),
        }
    }

    fn substitution(&self, key: &str) -> Option<Cow<'static, str>> {
        let example = include_str!("../doc/yml/substitution/example.md");
        match key {
            "append" => Some(include_str!("../doc/yml/substitution/append.md").into()),
            "ignorecase" => Some(include_str!("../doc/yml/substitution/ignorecase.md").into()),
            "nonword" => Some(include_str!("../doc/yml/substitution/nonword.md").into()),
            "exceptions" => Some(include_str!("../doc/yml/substitution/exceptions.md").into()),
            "swap" => Some(include_str!("../doc/yml/substitution/swap.md").into()),
            _ => self.common(key, example),
        }
    }

    fn occurrence(&self, key: &str) -> Option<Cow<'static, str>> {
        let example = include_str!("../doc/yml/occurrence/example.md");
        match key {
            "min" => Some(include_str!("../doc/yml/occurrence/min.md").into()),
            "max" => Some(include_str!("../doc/yml/occurrence/max.md").into()),
            "token" => Some(include_str!("../doc/yml/occurrence/token.md").into()),
            _ => self.common(key, example),
        }
    }

    fn repetition(&self, key: &str) -> Option<Cow<'static, str>> {
        let example = include_str!("../doc/yml/repetition/example.md");
        match key {
            "alpha" => Some(include_str!("../doc/yml/repetition/alpha.md").into()),
            "tokens" => Some(include_str!("../doc/yml/repetition/tokens.md").into()),
            _ => self.common(key, example),
        }
    }

    fn consistency(&self, key: &str) -> Option<Cow<'static, str>> {
        let example = include_str!("../doc/yml/consistency/example.md");
        match key {
            "either" => Some(include_str!("../doc/yml/consistency/either.md").into()),
            "nonword" => Some(include_str!("../doc/yml/consistency/nonword.md").into()),
            "ignorecase" => Some(include_str!("../doc/yml/consistency/ignorecase.md").into()),
            _ => self.common(key, example),
        }
    }

    fn conditional(&self, key: &str) -> Option<Cow<'static, str>> {
        let example = include_str!("../doc/yml/conditional/example.md");
        match key {
            "first" => Some(include_str!("../doc/yml/conditional/first.md").into()),
            "second" => Some(include_str!("../doc/yml/conditional/second.md").into()),
            "ignorecase" => Some(include_str!("../doc/yml/conditional/ignorecase.md").into()),
            _ => self.common(key, example),
        }
    }

    fn capitalization(&self, key: &str) -> Option<Cow<'static, str>> {
        let example = include_str!("../doc/yml/capitalization/example.md");
        match key {
            "exceptions" => Some(include_str!("../doc/yml/capitalization/exceptions.md").into()),
            "match" => Some(include_str!("../doc/yml/capitalization/match.md").into()),
            "style" => Some(include_str!("../doc/yml/capitalization/style.md").into()),
            _ => self.common(key, example),
        }
    }

    fn metric(&self, key: &str) -> Option<Cow<'static, str>> {
        let example = include_str!("../doc/yml/metric/example.md");
        match key {
            "formula" => Some(include_str!("../doc/yml/metric/formula.md").into()),
            "condition" => Some(include_str!("../doc/yml/metric/condition.md").into()),
            _ => self.common(key, example),
        }
    }

    fn spelling(&self, key: &str) -> Option<Cow<'static, str>> {
        let example = include_str!("../doc/yml/spelling/example.md");
        match key {
            "append" => Some(include_str!("../doc/yml/spelling/append.md").into()),
            "custom" => Some(include_str!("../doc/yml/spelling/custom.md").into()),
            "dicpath" => Some(include_str!("../doc/yml/spelling/dicpath.md").into()),
            "dictionaries" => Some(include_str!("../doc/yml/spelling/dictionaries.md").into()),
            "filters" => Some(include_str!("../doc/yml/spelling/filters.md").into()),
            "ignore" => Some(include_str!("../doc/yml/spelling/ignore.md").into()),
            _ => self.common(key, example),
        }
    }

    fn sequence(&self, key: &str) -> Option<Cow<'static, str>> {
        let example = include_str!("../doc/yml/sequence/example.md");
        match key {
            "ignorecase" => Some(include_str!("../doc/yml/sequence/ignorecase.md").into()),
            "tokens" => Some(include_str!("../doc/yml/sequence/tokens.md").into()),
            _ => self.common(key, example),
        }
    }

    fn script(&self, key: &str) -> Option<Cow<'static, str>> {
        let example = include_str!("../doc/yml/script/example.md");
        match key {
            "script" => Some(include_str!("../doc/yml/script/script.md").into()),
            _ => self.common(key, example),
        }
    }
}
