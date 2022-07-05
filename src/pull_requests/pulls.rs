use super::{PullListOptions, PullRequestCreateResponse, PullRequestsResponse};
use crate::{AzureClient, Future};

pub use super::pull::PullRequest;
pub use super::PROption;

pub struct PullRequests {
    ops: AzureClient,
    project: String,
    repo: String,
}

impl PullRequests {
    #[doc(hidden)]
    pub fn new<P, R>(ops: AzureClient, project: P, repo: R) -> Self
    where
        P: Into<String>,
        R: Into<String>,
    {
        Self {
            ops: ops,
            project: project.into(),
            repo: repo.into(),
        }
    }

    /// POST https://dev.azure.com/{organization}/{project}/_apis/git/repositories/{repositoryId}/pullrequests?api-version=5.1
    pub fn create(&self, options: &PROption) -> Future<PullRequestCreateResponse> {
        self.ops.post(&self.path(""), json!(options))
    }
    /// GET https://dev.azure.com/{organization}/{project}/_apis/git/repositories/{repositoryId}/pullrequests/{pullRequestId}?api-version=5.1
    pub fn pull(&self, id: u64) -> PullRequest {
        PullRequest::new(
            self.ops.clone(),
            self.project.as_str(),
            self.repo.as_str(),
            id,
        )
    }

    /// list pull requests
    pub fn list(&self, options: PullListOptions) -> Future<PullRequestsResponse> {
        let mut uri = vec![self.path("")];
        for (key, value) in options.params.into_iter() {
            uri.push(format!("{}={}", key, value))
        }
        let uri = &uri.join("?");
        self.ops.get::<PullRequestsResponse>(&format!("{}&", uri))
    }

    fn path(&self, more: &str) -> String {
        format!(
            "/{}/{}/_apis/git/repositories/{}/pullrequests{}",
            self.ops.org, self.project, self.repo, more
        )
    }
}
