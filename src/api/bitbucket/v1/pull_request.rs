use super::repository::Reference;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PullRequest {
    pub title: String,
    pub description: Option<String>,
    pub state: Option<String>,
    pub closed: bool,
    pub locked: bool,
    pub from_ref: Reference,
    pub to_ref: Reference,
    pub links: Option<Links>,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    pub _self: Vec<Link>     
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    pub href: String
}

#[cfg(test)]
mod tests {
    use super::PullRequest;
    use crate::api::bitbucket::v1::repository::{Project, Reference, Repository};

    #[test]
    fn pull_request_serialize() {
        let pr = PullRequest {
            title: String::from("title"),
            description: Some(String::from("description")),
            links: None,
            state: None,
            closed: false,
            locked: false,
            from_ref: Reference {
                id: String::from("from_id"),
                repository: Repository {
                    slug: String::from("from_slug"),
                    name: Option::None,
                    project: Project {
                        key: String::from("from_key"),
                    },
                },
            },
            to_ref: Reference {
                id: String::from("to_id"),
                repository: Repository {
                    slug: String::from("to_slug"),
                    name: Option::None,
                    project: Project {
                        key: String::from("to_key"),
                    },
                },
            },
        };
        let pr_in_json = serde_json::to_string(&pr).unwrap();
        let expected = r#"{"title":"title","description":"description","state":null,"closed":false,"locked":false,"fromRef":{"id":"from_id","repository":{"slug":"from_slug","name":null,"project":{"key":"from_key"}}},"toRef":{"id":"to_id","repository":{"slug":"to_slug","name":null,"project":{"key":"to_key"}}},"links":null}"#;
        assert_eq!(expected, pr_in_json);
    }
}
