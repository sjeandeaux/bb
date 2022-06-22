// curl -u <username>:$BITBUCKET_API_TOKEN https://<hostname>/rest/api/1.0/projects/<PROJECT>/repos/<REPO>/pull-requests -d @pr.json -H 'Content-type: application/json' -H 'Accept: application/json'
// curl -u <username>:$BITBUCKET_API_TOKEN https://<hostname>/rest/io.reconquest.bitbucket.labels/1.0/<PROJECT_ID>/<REPO_ID>/pull-requests/<PP_ID> -d "name=<LABEL>" -H "Content-Type: application/x-www-form-urlencoded" -X POST -H "X-Atlassian-Token: no-check"
//

mod api;
fn main() {}
