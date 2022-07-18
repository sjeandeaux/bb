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
        Some(("pr", pr_command)) => println!("pr: {:?}", pr_command.subcommand()),
        _ => unreachable!("clap should ensure we don't get here"),
    };
}

fn pr_command() -> clap::Command<'static> {
 clap::Command::new("pr").about("Manage pull requests").subcommand_required(true).subcommand(clap::SubCommand::with_name("create").about("Create a pull request").arg(clap::Arg::with_name("fill")))
}
