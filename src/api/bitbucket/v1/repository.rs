use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
    pub slug: String,
    pub name: Option<String>,
    pub project: Project,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reference {
    // "refs/heads/<branch name>"
    pub id: String,
    pub repository: Repository,
}

impl Repository {
    pub fn new(url: String) -> Self {
        let re = Regex::new(r"(http.?)://(.*)/scm/(.*)/(.*)\.git").unwrap();
        let caps = re.captures(&url).unwrap();
        Repository {
            slug: caps.get(4).map(|v| v.as_str().to_string()).unwrap(),
            name: caps.get(4).map(|v| v.as_str().to_string()),
            project: Project {
                key: caps.get(3).map(|v| v.as_str().to_string()).unwrap(),
            },
        }
    }
}
#[cfg(test)]
mod tests {
    use super::{Project, Reference, Repository};

    #[test]
    fn project_serialize() {
        let project = Project {
            key: String::from("test"),
        };
        let project_in_json = serde_json::to_string(&project).unwrap();
        let expected = r#"{"key":"test"}"#;
        assert_eq!(expected, project_in_json);
    }

    #[test]
    fn repository_without_name_serialize() {
        let repository = Repository {
            slug: String::from("slug"),
            name: Option::None,
            project: Project {
                key: String::from("key"),
            },
        };
        let repository_in_json = serde_json::to_string(&repository).unwrap();
        let expected = r#"{"slug":"slug","name":null,"project":{"key":"key"}}"#;
        assert_eq!(expected, repository_in_json);
    }

    #[test]
    fn repository_with_name_serialize() {
        let repository = Repository {
            slug: String::from("slug"),
            name: Option::from(String::from("name")),
            project: Project {
                key: String::from("key"),
            },
        };
        let repository_in_json = serde_json::to_string(&repository).unwrap();
        let expected = r#"{"slug":"slug","name":"name","project":{"key":"key"}}"#;
        assert_eq!(expected, repository_in_json);
    }

    #[test]
    fn reference_serialize() {
        let reference = Reference {
            id: String::from("id"),
            repository: Repository {
                slug: String::from("slug"),
                name: Option::None,
                project: Project {
                    key: String::from("key"),
                },
            },
        };
        let reference_in_json = serde_json::to_string(&reference).unwrap();
        let expected =
            r#"{"id":"id","repository":{"slug":"slug","name":null,"project":{"key":"key"}}}"#;
        assert_eq!(expected, reference_in_json);
    }
}
