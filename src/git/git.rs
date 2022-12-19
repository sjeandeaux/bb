use std::{error::Error, process::Command};

#[derive(Debug, PartialEq)]
/// A commit its sha and its title
pub struct Commit {
    pub sha: String,
    pub title: String,
}

impl Commit {
    /// Returns the body commit
    ///
    /// # Examples
    ///
    /// ```
    /// use
    /// ```
    pub fn commit_body(&self) -> Result<String, Box<dyn Error>> {
        let sha = &(self).sha;
        return lookup_commit(sha.to_string(), String::from("%b"));
    }
}

pub fn last_commit_sha() -> Result<Commit, Box<dyn Error>> {
    let tmp = lookup_commit(String::from("HEAD"), String::from("%H,%s"))?;
    let mut commit_and_title = tmp.split(",");
    return Ok(Commit {
        // TODO should manage the option in a proper way
        sha: commit_and_title.next().unwrap().to_string(),
        title: commit_and_title.next().unwrap().to_string(),
    });
}

pub fn current_branch() -> Result<String, Box<dyn Error>> {
    let output = Command::new("git")
        .arg("symbolic-ref")
        .arg("HEAD")
        .output()?;

    if output.status.success() {
        let mut stdout = String::from_utf8(output.stdout).unwrap();
        stdout.pop();
        return Ok(stdout);
    }
    return Err("fail to execute git".into());
}

pub fn get_remote_origin_url() -> Result<String, Box<dyn Error>> {
    let output = Command::new("git")
        .arg("config")
        .arg("--get")
        .arg("remote.origin.url")
        .output()?;

    if output.status.success() {
        let mut stdout = String::from_utf8(output.stdout).unwrap();
        stdout.pop();
        return Ok(stdout);
    }
    return Err("fail to execute git".into());
}

fn lookup_commit(sha: String, format: String) -> Result<String, Box<dyn Error>> {
    let output = Command::new("git")
        .arg("-c")
        .arg("log.ShowSignature=false")
        .arg("show")
        .arg("-s")
        .arg(format!("--pretty=format:{format}"))
        .arg(sha)
        .output()?;

    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).unwrap();
        return Ok(stdout);
    }
    return Err("fail to execute git".into());
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::{current_branch, get_remote_origin_url, last_commit_sha, Commit};

    #[test]
    fn test_last_commit_sha() {
        env::set_var("GIT_DIR", "./fixtures/simple.git");
        let commit = last_commit_sha().expect("should not fail");
        assert_eq!(
            commit,
            Commit {
                sha: String::from("9233473ecfc9e45a58b1184dbe9da9758d51ab63"),
                title: String::from("title")
            }
        )
    }

    #[test]
    fn test_current_branch() {
        env::set_var("GIT_DIR", "./fixtures/simple.git");
        let branch = current_branch().expect("should not fail");
        assert_eq!(branch, String::from("refs/heads/master"))
    }

    #[test]
    fn test_get_remote_origin_url() {
        env::set_var("GIT_DIR", "./fixtures/simple.git");
        let branch = get_remote_origin_url().expect("should not fail");
        assert_eq!(branch, String::from("simple.git"))
    }
    #[test]
    fn test_commit_body() {
        env::set_var("GIT_DIR", "./fixtures/simple.git");
        let commit_body = last_commit_sha()
            .expect("should not fail")
            .commit_body()
            .expect("should not fail");
        assert_eq!(commit_body, String::from("multi line\n\nend\n"))
    }
}
