// TODO mod and use double git
mod api;
mod git;
use api::bitbucket::v1::pull_request::PullRequest;
use clap::ArgAction;
use git::git::{current_branch, get_remote_origin_url, last_commit_sha};
use reqwest::{Error, Response, StatusCode};
// curl -u <username>:$BITBUCKET_API_TOKEN https://<hostname>/rest/api/1.0/projects/<PROJECT>/repos/<REPO>/pull-requests -d @pr.json -H 'Content-type: application/json' -H 'Accept: application/json'
// curl -u <username>:$BITBUCKET_API_TOKEN https://<hostname>/rest/io.reconquest.bitbucket.labels/1.0/<PROJECT_ID>/<REPO_ID>/pull-requests/<PP_ID> -d "name=<LABEL>" -H "Content-Type: application/x-www-form-urlencoded" -X POST -H "X-Atlassian-Token: no-check"
//

#[tokio::main]
async fn main() {
    let cmd = clap::Command::new("bb")
        .bin_name("bb")
        .subcommand_required(true)
        .subcommand(pr_command());
    let principale = cmd.get_matches();
    match principale.subcommand() {
        Some(("pr", pr_command)) => {
            let r = pr_actions(pr_command).await.unwrap();
            match r.status(){
                StatusCode::CREATED => {
                    let pr = r.json::<PullRequest>().await.unwrap();
                    println!("{}", pr.links.expect("missing link")._self.first().expect("missing href").href);
                }
                s => println!("{} {}", s,  std::str::from_utf8(&r.bytes().await.unwrap()).unwrap())
            }
                            
        }
        _ => unreachable!("clap should ensure we don't get here"),
    };
}

async fn pr_actions(pr_command: &clap::ArgMatches) -> Result<Response, Error> {
    match pr_command.subcommand() {
        Some(("create", create_command)) => pr_create_action(create_command).await,
        _ => unreachable!("clap should ensure we don't get here"),
    }
}

async fn pr_create_action(create_command: &clap::ArgMatches) -> Result<Response, Error> {
    let last_commit = last_commit_sha().expect("git error");

    //TODO those information should come from git
    let client = api::bitbucket::v1::client::Client::new(
        get_remote_origin_url().expect("missing remote url"),
        create_command
            .get_one::<String>("username")
            .expect("missing username")
            .to_string(),
        create_command
            .get_one::<String>("token")
            .expect("missing token")
            .to_string(),
    );

    let repository_from = api::bitbucket::v1::repository::Repository::new(
        get_remote_origin_url().expect("missing remote url"),
    );
    let repository_to = api::bitbucket::v1::repository::Repository::new(
        get_remote_origin_url().expect("missing remote url"),
    );

    let pr = api::bitbucket::v1::pull_request::PullRequest {
        title: last_commit.title.to_owned(),
        description: Some(last_commit.commit_body().expect("cannot get body commit")),
        state: None,
        links: None,
        closed: false,
        locked: false,
        from_ref: api::bitbucket::v1::repository::Reference {
            id: format!(
                "refs/heads/{branch}",
                branch = create_command
                    .get_one::<String>("branch")
                    .expect("missing branch")
                    .to_string()
            ),
            repository: repository_from,
        },
        to_ref: api::bitbucket::v1::repository::Reference {
            id: current_branch().expect("unable to get the current branch"),
            repository: repository_to,
        },
    };

    client.create_pull_request(&pr).await
}

fn pr_command() -> clap::Command {
    clap::Command::new("pr")
        .about("Manage pull requests")
        .subcommand_required(true)
        .subcommand(
            clap::Command::new("create")
                .arg(
                    clap::Arg::new("username")
                        .long("username")
                        .env("BB_USERNAME")
                        .action(ArgAction::Set),
                )
                .arg(
                    clap::Arg::new("token")
                        .long("token")
                        .env("BB_TOKEN")
                        .action(ArgAction::Set),
                )
                .arg(
                    clap::Arg::new("branch")
                        .long("branch")
                        .default_value("develop"),
                )
                .about("Create a pull request"),
        )
}
