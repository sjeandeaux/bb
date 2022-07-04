use super::pull_request::PullRequest;
use regex::Regex;
use reqwest::Response;

pub struct Client {
    pub protocol: Option<String>,
    pub hostname: String,

    pub username: String,
    pub token: String,
}

impl Client {
    pub fn new(url: String, username: String, token: String) -> Self {
        let re = Regex::new(r"(http.?)://(.*)/scm/(.*)/(.*)\.git").unwrap();
        let caps = re.captures(&url).unwrap();
        Self {
            protocol: caps.get(1).map(|v| v.as_str().to_string()),
            hostname: caps.get(2).map(|v| v.as_str().to_string()).unwrap(),
            username,
            token,
        }
    }
    // TODO refactoring argument(s)
    fn endpoint_pull_request(&self, pull_request: &PullRequest) -> String {
        return format!("{protocol}://{hostname}/rest/api/1.0/projects/{project}/repos/{repository}/pull-requests", 
                            protocol = self.protocol.as_ref().unwrap_or(&String::from("https")),
                            hostname = self.hostname,
                            project = pull_request.to_ref.repository.project.key,
                            repository = pull_request.to_ref.repository.name.as_ref().unwrap());
    }

    pub async fn create_pull_request(
        &self,
        pull_request: &PullRequest,
    ) -> Result<Response, reqwest::Error> {
        let client = reqwest::Client::new();
        return client
            .post(self.endpoint_pull_request(pull_request))
            .basic_auth(&self.username, Some(&self.token))
            .json(pull_request)
            .send()
            .await;
    }
}

#[cfg(test)]
mod tests {
    use crate::api::bitbucket::v1::{
        pull_request::PullRequest,
        repository::{Project, Reference, Repository},
    };

    use super::Client;

    #[test]
    fn endpoint_pull_request() {
        let client = Client {
            protocol: None,
            hostname: String::from("hostname"),
            username: String::from("username"),
            token: String::from("token"),
        };
        let pr = PullRequest {
            title: String::from("title"),
            links: None,
            description: Some(String::from("description")),
            state: None,
            closed: false,
            locked: false,
            from_ref: Reference {
                id: String::from("from_id"),
                repository: Repository {
                    slug: String::from("from_slug"),
                    name: Option::Some(String::from("from_name")),
                    project: Project {
                        key: String::from("from_key"),
                    },
                },
            },
            to_ref: Reference {
                id: String::from("to_id"),
                repository: Repository {
                    slug: String::from("to_slug"),
                    name: Option::Some(String::from("to_name")),
                    project: Project {
                        key: String::from("to_key"),
                    },
                },
            },
        };
        assert_eq!(
            client.endpoint_pull_request(&pr),
            String::from(
                "https://hostname/rest/api/1.0/projects/to_key/repos/to_name/pull-requests"
            )
        )
    }
}
