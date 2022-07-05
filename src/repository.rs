use crate::{AzureClient, Future};

use crate::pull_requests::{PullRequest, PullRequests};
pub use repository_create_options::RepoOptions;
pub use repository_create_response::RepoCreateReponse;
pub use repository_list_options::RepoListOptions;
pub use repository_list_response::ReposResponse;
pub use repository_response::RepoResponse;

pub struct Repository {
    ops: AzureClient,
    project: String,
    repo: String,
}

impl Repository {
    #[doc(hidden)]
    pub fn new<P, R>(ops: AzureClient, project: P, repo: R) -> Self
    where
        P: Into<String>,
        R: Into<String>,
    {
        Repository {
            ops,
            project: project.into(),
            repo: repo.into(),
        }
    }

    /// DELETE https://dev.azure.com/{organization}/{project}/_apis/git/repositories/{repositoryId}?api-version=5.1
    pub fn delete(&self) -> Future<()> {
        self.ops.delete(&self.path(""))
    }

    /// Get pull requests ref
    pub fn pulls(&self) -> PullRequests {
        PullRequests::new(self.ops.clone(), self.project.as_str(), self.repo.as_str())
    }

    /// Short hand to get a specific pr directly
    pub fn pull(&self, id: u64) -> PullRequest {
        PullRequest::new(
            self.ops.clone(),
            self.project.as_str(),
            self.repo.as_str(),
            id,
        )
    }

    fn path(&self, more: &str) -> String {
        format!(
            "/{}/{}/_apis/git/repositories/{}{}",
            self.ops.org, self.project, self.repo, more
        )
    }

    /// GET https://dev.azure.com/{organization}/{project}/_apis/git/repositories/{repositoryId}?api-version=5.1
    pub fn get(&self) -> Future<RepoResponse> {
        self.ops.get(&self.path(""))
    }
}

mod repository_response {
    use serde::*;
    #[derive(Debug, Deserialize)]
    pub struct RepoResponse {
        pub id: String,
        pub name: String,
        pub url: String,
        pub project: Project,
        #[serde(rename = "defaultBranch")]
        pub default_branch: String,
        pub size: i64,
        #[serde(rename = "remoteUrl")]
        pub remote_url: String,
        #[serde(rename = "sshUrl")]
        pub ssh_url: String,
        #[serde(rename = "webUrl")]
        pub web_url: String,
        #[serde(rename = "_links")]
        pub links: Links,
    }
    #[derive(Debug, Deserialize)]
    pub struct Project {
        pub id: String,
        pub name: String,
        pub url: String,
        pub state: String,
        pub revision: i64,
        pub visibility: String,
        #[serde(rename = "lastUpdateTime")]
        pub last_update_time: String,
    }
    #[derive(Debug, Deserialize)]
    pub struct Links {
        #[serde(rename = "self")]
        pub self_field: Href,
        pub project: ProjectHref,
        pub web: Web,
        pub ssh: Ssh,
        pub commits: Commits,
        pub refs: Refs,
        #[serde(rename = "pullRequests")]
        pub pull_requests: PullRequests,
        pub items: Items,
        pub pushes: Pushes,
    }
    #[derive(Debug, Deserialize)]
    pub struct Href {
        pub href: String,
    }
    #[derive(Debug, Deserialize)]
    pub struct ProjectHref {
        pub href: String,
    }
    #[derive(Debug, Deserialize)]
    pub struct Web {
        pub href: String,
    }
    #[derive(Debug, Deserialize)]
    pub struct Ssh {
        pub href: String,
    }
    #[derive(Debug, Deserialize)]
    pub struct Commits {
        pub href: String,
    }
    #[derive(Debug, Deserialize)]
    pub struct Refs {
        pub href: String,
    }
    #[derive(Debug, Deserialize)]
    pub struct PullRequests {
        pub href: String,
    }
    #[derive(Debug, Deserialize)]
    pub struct Items {
        pub href: String,
    }
    #[derive(Debug, Deserialize)]
    pub struct Pushes {
        pub href: String,
    }
}

pub struct Repositories {
    ops: AzureClient,
    project: String,
}

impl Repositories {
    pub fn new<P>(ops: AzureClient, project: P) -> Self
    where
        P: Into<String>,
    {
        Self {
            ops: ops,
            project: project.into(),
        }
    }

    /// Create a new repository
    pub fn create(&self, repo: &RepoOptions) -> Future<RepoCreateReponse> {
        self.ops.post(&self.path(""), json!(repo))
    }

    fn path(&self, more: &str) -> String {
        format!(
            "/{}/{}/_apis/git/repositories{}",
            self.ops.org, self.project, more
        )
    }

    /// list the authenticated users repositories
    ///
    /// https://docs.microsoft.com/en-us/rest/api/azure/devops/git/repositories/list?view=azure-devops-rest-5.1
    pub fn list(&self, options: &RepoListOptions) -> Future<ReposResponse> {
        let mut uri = vec![self.path("")];
        if let Some(query) = options.serialize() {
            uri.push(query);
        }
        self.ops.get(&uri.join("?"))
    }
}

mod repository_create_options {
    use serde::*;
    #[derive(Debug, Default, Serialize)]
    pub struct RepoOptions {
        pub name: String,
    }

    pub struct RepoOptionsBuilder(RepoOptions);

    impl RepoOptionsBuilder {
        pub(crate) fn new<N>(name: N) -> Self
        where
            N: Into<String>,
        {
            RepoOptionsBuilder(RepoOptions {
                name: name.into(),
                ..Default::default()
            })
        }
        pub fn build(&self) -> RepoOptions {
            RepoOptions::new(self.0.name.as_str() /*,self.0.project.id.clone()*/)
        }
    }

    impl RepoOptions {
        #[allow(clippy::too_many_arguments)] // exempted
        pub fn new<N>(name: N) -> Self
        where
            N: Into<String>,
        {
            RepoOptions { name: name.into() }
        }

        pub fn builder<N: Into<String>>(name: N) -> RepoOptionsBuilder {
            RepoOptionsBuilder::new(name)
        }
    }
}

mod repository_create_response {
    use serde::*;
    #[derive(Debug, Deserialize)]
    pub struct RepoCreateReponse {
        pub id: String,
        pub name: String,
        pub url: String,
        pub project: Project,
        pub size: i64,
        #[serde(rename = "remoteUrl")]
        pub remote_url: String,
        #[serde(rename = "sshUrl")]
        pub ssh_url: String,
        #[serde(rename = "webUrl")]
        pub web_url: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct Project {
        pub id: String,
        pub name: String,
        pub url: String,
        pub state: String,
        pub revision: i64,
        pub visibility: String,
        #[serde(rename = "lastUpdateTime")]
        pub last_update_time: String,
    }
}

mod repository_list_options {
    use std::collections::HashMap;
    use url::form_urlencoded;

    #[derive(Default)]
    pub struct RepoListOptions {
        params: HashMap<&'static str, String>,
    }

    impl RepoListOptions {
        pub fn builder() -> RepoListOptionsBuilder {
            RepoListOptionsBuilder::default()
        }

        /// serialize options as a string. returns None if no options are defined
        pub fn serialize(&self) -> Option<String> {
            if self.params.is_empty() {
                None
            } else {
                let encoded: String = form_urlencoded::Serializer::new(String::new())
                    .extend_pairs(&self.params)
                    .finish();
                Some(encoded)
            }
        }
    }

    #[derive(Default)]
    pub struct RepoListOptionsBuilder(RepoListOptions);

    impl RepoListOptionsBuilder {
        pub fn include_hidden(&mut self, include_hidden: bool) -> &mut Self {
            self.0
                .params
                .insert("includeHidden", include_hidden.to_string());
            self
        }

        pub fn include_links(&mut self, include_links: bool) -> &mut Self {
            self.0
                .params
                .insert("includeLinks", include_links.to_string());
            self
        }

        pub fn include_all_urls(&mut self, include_all_urls: bool) -> &mut Self {
            self.0
                .params
                .insert("includeAllUrls", include_all_urls.to_string());
            self
        }

        pub fn build(&self) -> RepoListOptions {
            RepoListOptions {
                params: self.0.params.clone(),
            }
        }
    }
}

mod repository_list_response {
    use serde::*;
    #[derive(Debug, Deserialize)]
    pub struct ReposResponse {
        pub value: Vec<Value>,
        pub count: i64,
    }
    #[derive(Debug, Deserialize)]
    pub struct Value {
        pub id: String,
        pub name: String,
        pub url: String,
        pub project: Project,
        #[serde(rename = "defaultBranch")]
        pub default_branch: Option<String>,
        pub size: i64,
        #[serde(rename = "remoteUrl")]
        pub remote_url: String,
        #[serde(rename = "sshUrl")]
        pub ssh_url: String,
        #[serde(rename = "webUrl")]
        pub web_url: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct Project {
        pub id: String,
        pub name: String,
        pub url: String,
        pub state: String,
        pub revision: i64,
        pub visibility: String,
        #[serde(rename = "lastUpdateTime")]
        pub last_update_time: String,
        pub description: Option<String>,
    }
}
