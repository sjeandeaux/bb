use std::process::Command;

#[derive(Debug)]
pub struct Commit {
    pub sha: String,
    pub title: String,
}

pub fn last_commit_sha() -> Commit {
    let tmp = lookup_commit(String::from("HEAD"), String::from("%H,%s"));
    let mut commit_and_title = tmp.split(",");
    return Commit {
        sha: commit_and_title.next().unwrap().to_string(),
        title: commit_and_title.next().unwrap().to_string(),
    };
}

impl Commit {
    pub fn commit_body(self) -> String {
        return lookup_commit(self.sha, String::from("%b"));
    }
}

fn lookup_commit(sha: String, format: String) -> String {
    let output = Command::new("git")
        .arg("-c")
        .arg("log.ShowSignature=false")
        .arg("show")
        .arg("-s")
        .arg(format!("--pretty=format:{format}"))
        .arg(sha)
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8(output.stdout).unwrap();
    return stdout;
}
