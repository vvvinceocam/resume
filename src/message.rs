use std::str::FromStr;

use pest::Parser;
use pest_derive::Parser;

/// Parsed commit message following [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/)
/// convention.
#[derive(Debug)]
pub struct ConventionalMessage {
    pub ctype: CommitType,
    pub scope: Option<String>,
    pub is_breaking: bool,
    pub summary: String,
    pub body: Option<String>,
    pub trailers: Vec<(String, String)>,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum CommitType {
    ContinuousIntegration,
    Build,
    BugFix,
    Documentation,
    Feature,
    Performance,
    Refactoring,
    Style,
    Test,
    Other(String),
}

/// PEG parser based on Pest definition
#[derive(Parser)]
#[grammar = "conventional_message.pest"]
struct ConventionalMessageParser;

impl FromStr for ConventionalMessage {
    type Err = pest::error::Error<Rule>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parser = ConventionalMessageParser::parse(Rule::message, s)?;
        let mut message = ConventionalMessage {
            ctype: CommitType::Other("".to_owned()),
            scope: None,
            is_breaking: false,
            summary: "".to_string(),
            body: None,
            trailers: vec![],
        };

        let pairs = parser.next().unwrap().into_inner();

        // Parse headline/body/trailers
        for pair in pairs.clone() {
            match pair.as_rule() {
                Rule::headline => {
                    let pairs = pair.clone().into_inner();

                    // Parse ctype/scope/break_mark/summary
                    for pair in pairs {
                        match pair.as_rule() {
                            Rule::ctype => {
                                message.ctype = pair.as_str().parse().expect("unfailable")
                            }
                            Rule::scope => message.scope = Some(pair.as_str().to_owned()),
                            Rule::summary => message.summary = pair.as_str().to_owned(),
                            Rule::break_mark => message.is_breaking = true,
                            _ => unreachable!(),
                        }
                    }
                }
                Rule::body => {
                    message.body = Some(pair.as_str().to_owned());
                }
                Rule::trailers => {
                    let pairs = pair.clone().into_inner();
                    for pair in pairs {
                        let mut pairs = pair.clone().into_inner();
                        let token = pairs
                            .next()
                            .expect("broken parser: MUST have token")
                            .as_str()
                            .to_owned();
                        let value = pairs
                            .next()
                            .expect("broken parser: MUST have value")
                            .as_str()
                            .to_owned();
                        message.trailers.push((token, value));
                    }
                }
                _ => unreachable!(),
            }
        }

        Ok(message)
    }
}

impl FromStr for CommitType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "build" => Self::Build,
            "ci" => Self::ContinuousIntegration,
            "docs" => Self::Documentation,
            "feat" => Self::Feature,
            "fix" => Self::BugFix,
            "perf" => Self::Performance,
            "refactor" => Self::Refactoring,
            "style" => Self::Style,
            "test" => Self::Test,
            s => Self::Other(s.to_owned()),
        })
    }
}