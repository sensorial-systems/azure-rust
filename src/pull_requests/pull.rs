use super::{PullRequestResponse, PullStatus, PullUpdateOptions};
use crate::{AzureClient, Future};

pub struct PullRequest {
    ops: AzureClient,
    project: String,
    repo: String,
    id: u64,
}

impl PullRequest {
    #[doc(hidden)]
    pub fn new<P, R>(ops: AzureClient, project: P, repo: R, id: u64) -> Self
    where
        P: Into<String>,
        R: Into<String>,
    {
        Self {
            ops: ops,
            project: project.into(),
            repo: repo.into(),
            id: id,
        }
    }

    /// https://dev.azure.com/bentestingacc/Azure-Testing/_apis/git/repositories/064ad20f-f240-4747-8cc0-057175cab664/pullrequests?api-version=5.1
    fn path(&self, more: &str) -> String {
        format!(
            "/{}/{}/_apis/git/repositories/{}/pullrequests/{}{}",
            self.ops.org, self.project, self.repo, self.id, more
        )
    }
    /// Request a pull requests information
    pub fn get(&self) -> Future<PullRequestResponse> {
        self.ops.get(&self.path(""))
    }

    /// Update a pull request
    pub fn update(&self, pr: &PullUpdateOptions) -> Future<PullRequestResponse> {
        let body = json!(pr);
        self.ops
            .patch::<PullRequestResponse>(&self.path("?"), body)
    }

    /// short hand for updating pr status = active
    pub fn active(&self) -> Future<PullRequestResponse> {
        self.update(&PullUpdateOptions::builder().status(PullStatus::Active).build())
    }

    /// short hand for updating pr status = abandoned
    pub fn abandon(&self) -> Future<PullRequestResponse> {
        self.update(&PullUpdateOptions::builder().status(PullStatus::Abandoned).build())
    }

}
