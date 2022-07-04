use super::pull_request::PullRequest;

pub struct Client {
    pub protocol: Option<String>,
    pub hostname: String,

    pub username: String,
    pub token: String,
}

impl Client {
    // TODO refactoring argument(s)
    pub fn endpoint_pull_request(&self, pull_request: &PullRequest) -> String {
        return format!("{protocol}://{hostname}/rest/api/1.0/projects/{project}/repos/{repository}/pull-requests", 
                            protocol = self.protocol.as_ref().unwrap_or(&String::from("https")),
                            hostname = self.hostname,
                            project = pull_request.to_ref.repository.project.key,
                            repository = pull_request.to_ref.repository.name.as_ref().unwrap());
    }

    pub async fn create_pull_request(
        &self,
        pull_request: &PullRequest,
    ) -> Result<PullRequest, reqwest::Error> {
        let client = reqwest::Client::new();
        let res: PullRequest = client
            .post(self.endpoint_pull_request(pull_request))
            .basic_auth(&self.username, Some(&self.token))
            .json(&pull_request)
            .send()
            .await?
            .json()
            .await?;

        return Ok(res);
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
        // TODO rust and factory ???
        let pr = PullRequest {
            title: String::from("title"),
            description: String::from("description"),
            state: true,
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
