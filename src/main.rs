// TODO mod and use double git
mod git;
use git::git::last_commit_sha;

// curl -u <username>:$BITBUCKET_API_TOKEN https://<hostname>/rest/api/1.0/projects/<PROJECT>/repos/<REPO>/pull-requests -d @pr.json -H 'Content-type: application/json' -H 'Accept: application/json'
// curl -u <username>:$BITBUCKET_API_TOKEN https://<hostname>/rest/io.reconquest.bitbucket.labels/1.0/<PROJECT_ID>/<REPO_ID>/pull-requests/<PP_ID> -d "name=<LABEL>" -H "Content-Type: application/x-www-form-urlencoded" -X POST -H "X-Atlassian-Token: no-check"
//

fn main() {
    let cmd = clap::Command::new("bb")
        .bin_name("bb")
        .subcommand_required(true)
        .subcommand(pr_command());
    let principale = cmd.get_matches();
    match principale.subcommand() {
        Some(("pr", pr_command)) => pr_actions(pr_command),
        _ => unreachable!("clap should ensure we don't get here"),
    };
}

fn pr_actions(pr_command: &clap::ArgMatches) -> () {
    match pr_command.subcommand() {
        Some(("create", create_command)) => pr_create_action(create_command),
        _ => unreachable!("clap should ensure we don't get here"),
    }
}

fn pr_create_action(create_command: &clap::ArgMatches) {
    if !create_command.is_present("fill") {
        todo!("should be handled")
    }
    let last_commit = last_commit_sha().expect("git error");
    println!("{:?} {:?}", last_commit, last_commit.commit_body())
}

fn pr_command() -> clap::Command<'static> {
    clap::Command::new("pr")
        .about("Manage pull requests")
        .subcommand_required(true)
        .subcommand(
            clap::SubCommand::with_name("create")
                .arg(clap::Arg::with_name("fill").long("fill").short('f'))
                .about("Create a pull request"),
        )
}
