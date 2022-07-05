use std::fmt;

use serde::Serialize;

pub use pr_create_options::PROption;
pub use pr_create_response::PullRequestCreateResponse;
pub use pr_get_response::PullRequestResponse;
pub use pull_request_list_options::{PullListOptions, PullListOptionsBuilder};
pub use pull_requests_reponse::PullRequestsResponse;
pub use pull_update_options::{PullUpdateOptions, PullUpdateOptionsBuilder};

pub mod pull;
pub mod pulls;

pub use pull::PullRequest;
pub use pulls::PullRequests;

mod pr_get_response {
    // use super::pr_refs::LinksRefs;
    use serde::*;

    #[derive(Debug, Deserialize)]
    pub struct PullRequestResponse {
        // #[serde(flatten)]
        // pub repository: Repository,
        #[serde(rename = "pullRequestId")]
        pub pull_request_id: i64,
        // #[serde(rename = "codeReviewId")]
        // pub code_review_id: Option<i64>,
        // pub status: Option<String>,
        #[serde(rename = "createdBy")]
        pub created_by: CreatedBy,
        // #[serde(rename = "creationDate")]
        // pub creation_date: String,
        // pub title: String,
        // pub description: Option<String>,
        // #[serde(rename = "sourceRefName")]
        // pub source_ref_name: String,
        // #[serde(rename = "targetRefName")]
        // pub target_ref_name: String,
        // #[serde(rename = "mergeStatus")]
        // pub merge_status: String,
        // #[serde(rename = "isDraft")]
        // pub is_draft: bool,
        // #[serde(rename = "mergeId")]
        // pub merge_id: String,
        // #[serde(rename = "lastMergeSourceCommit")]
        // pub last_merge_source_commit: LastMergeSourceCommit,
        // #[serde(rename = "lastMergeTargetCommit")]
        // pub last_merge_target_commit: LastMergeTargetCommit,
        // #[serde(rename = "lastMergeCommit")]
        // pub last_merge_commit: LastMergeCommit,
        // pub reviewers: Vec<::serde_json::Value>,
        // pub url: String,
        // #[serde(rename = "_links")]
        // pub links: LinksRefs,
        // #[serde(rename = "supportsIterations")]
        // pub supports_iterations: bool,
        // #[serde(rename = "artifactId")]
        // pub artifact_id: String,
    }
    #[derive(Debug, Deserialize)]
    pub struct Repository {
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
    #[derive(Debug, Deserialize)]
    pub struct CreatedBy {
        #[serde(rename = "displayName")]
        pub display_name: String,
        pub url: String,
        // #[serde(rename = "_links")]
        // pub links: Links,
        pub id: String,
        #[serde(rename = "uniqueName")]
        pub unique_name: String,
        #[serde(rename = "imageUrl")]
        pub image_url: String,
        pub descriptor: String,
    }
    #[derive(Debug, Deserialize)]
    pub struct Links {
        pub avatar: Avatar,
    }
    #[derive(Debug, Deserialize)]
    pub struct Avatar {
        pub href: String,
    }
    #[derive(Debug, Deserialize)]
    pub struct LastMergeSourceCommit {
        #[serde(rename = "commitId")]
        pub commit_id: String,
        pub url: String,
    }
    #[derive(Debug, Deserialize)]
    pub struct LastMergeTargetCommit {
        #[serde(rename = "commitId")]
        pub commit_id: String,
        pub url: String,
    }
    #[derive(Debug, Deserialize)]
    pub struct LastMergeCommit {
        #[serde(rename = "commitId")]
        pub commit_id: String,
        pub author: Author,
        pub committer: Committer,
        pub comment: String,
        pub url: String,
    }
    #[derive(Debug, Deserialize)]
    pub struct Author {
        pub name: String,
        pub email: String,
        pub date: String,
    }
    #[derive(Debug, Deserialize)]
    pub struct Committer {
        pub name: String,
        pub email: String,
        pub date: String,
    }
}

mod pr_create_response {
    use super::pr_refs::LinksRefs;
    use serde::*;

    #[derive(Debug, Deserialize)]
    pub struct PullRequestCreateResponse {
        pub repository: Repository,
        #[serde(rename = "pullRequestId")]
        pub pull_request_id: i64,
        #[serde(rename = "codeReviewId")]
        pub code_review_id: i64,
        pub status: String,
        #[serde(rename = "createdBy")]
        pub created_by: CreatedBy,
        #[serde(rename = "creationDate")]
        pub creation_date: String,
        pub title: String,
        pub description: String,
        #[serde(rename = "sourceRefName")]
        pub source_ref_name: String,
        #[serde(rename = "targetRefName")]
        pub target_ref_name: String,
        #[serde(rename = "mergeStatus")]
        pub merge_status: String,
        #[serde(rename = "isDraft")]
        pub is_draft: bool,
        #[serde(rename = "mergeId")]
        pub merge_id: String,
        #[serde(rename = "lastMergeSourceCommit")]
        pub last_merge_source_commit: LastMergeSourceCommit,
        #[serde(rename = "lastMergeTargetCommit")]
        pub last_merge_target_commit: LastMergeTargetCommit,
        pub reviewers: Vec<::serde_json::Value>,
        pub labels: Vec<::serde_json::Value>,
        pub url: String,
        #[serde(rename = "_links")]
        pub links: LinksRefs,
        #[serde(rename = "supportsIterations")]
        pub supports_iterations: bool,
        #[serde(rename = "artifactId")]
        pub artifact_id: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct Repository {
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

    #[derive(Debug, Deserialize)]
    pub struct CreatedBy {
        #[serde(rename = "displayName")]
        pub display_name: String,
        pub url: String,
        #[serde(rename = "_links")]
        pub links: Links,
        pub id: String,
        #[serde(rename = "uniqueName")]
        pub unique_name: String,
        #[serde(rename = "imageUrl")]
        pub image_url: String,
        pub descriptor: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct Links {
        pub avatar: Avatar,
    }

    #[derive(Debug, Deserialize)]
    pub struct Avatar {
        pub href: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct LastMergeSourceCommit {
        #[serde(rename = "commitId")]
        pub commit_id: String,
        pub url: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct LastMergeTargetCommit {
        #[serde(rename = "commitId")]
        pub commit_id: String,
        pub url: String,
    }
}

mod pr_refs {
    use serde::*;
    #[derive(Debug, Deserialize)]
    pub struct LinksRefs {
        #[serde(rename = "self")]
        pub self_field: Href,
        pub repository: Option<RepositoryRef>,
        #[serde(rename = "workItems")]
        pub work_items: Option<WorkItemsRef>,
        #[serde(rename = "sourceBranch")]
        pub source_branch: Option<SourceBranchRef>,
        #[serde(rename = "targetBranch")]
        pub target_branch: Option<TargetBranchRef>,
        pub statuses: Option<StatusesRef>,
        #[serde(rename = "sourceCommit")]
        pub source_commit: Option<SourceCommitRef>,
        #[serde(rename = "targetCommit")]
        pub target_commit: Option<TargetCommitRef>,
        #[serde(rename = "createdBy")]
        pub created_by: Option<CreatedByRef>,
        pub iterations: Option<IterationsRef>,
    }
    #[derive(Debug, Deserialize)]
    pub struct Href {
        pub href: String,
    }
    #[derive(Debug, Deserialize)]
    pub struct RepositoryRef {
        pub href: String,
    }
    #[derive(Debug, Deserialize)]
    pub struct WorkItemsRef {
        pub href: String,
    }
    #[derive(Debug, Deserialize)]
    pub struct SourceBranchRef {
        pub href: String,
    }
    #[derive(Debug, Deserialize)]
    pub struct TargetBranchRef {
        pub href: String,
    }
    #[derive(Debug, Deserialize)]
    pub struct StatusesRef {
        pub href: String,
    }
    #[derive(Debug, Deserialize)]
    pub struct SourceCommitRef {
        pub href: String,
    }
    #[derive(Debug, Deserialize)]
    pub struct TargetCommitRef {
        pub href: String,
    }
    #[derive(Debug, Deserialize)]
    pub struct CreatedByRef {
        pub href: String,
    }
    #[derive(Debug, Deserialize)]
    pub struct IterationsRef {
        pub href: String,
    }
}

mod pull_request_list_options {

    use super::PullStatus;
    use std::collections::HashMap;
    use url::form_urlencoded;
    #[derive(Default)]
    pub struct PullListOptions {
        pub params: HashMap<&'static str, String>,
    }

    impl PullListOptions {
        pub fn builder() -> PullListOptionsBuilder {
            PullListOptionsBuilder::default()
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
    pub struct PullListOptionsBuilder(PullListOptions);

    impl PullListOptionsBuilder {
        pub fn skip(&mut self, skip: u32) -> &mut Self {
            self.0.params.insert("skip", skip.to_string());
            self
        }

        pub fn top(&mut self, top: u32) -> &mut Self {
            self.0.params.insert("top", top.to_string());
            self
        }

        pub fn direction<D>(&mut self, direction: D) -> &mut Self
        where
            D: Into<String>,
        {
            self.0.params.insert("direction", direction.into());
            self
        }

        pub fn status(&mut self, status: PullStatus) -> &mut Self {
            self.0
                .params
                .insert("searchCriteria.status", status.to_string());
            self
        }

        pub fn repository_id<R>(&mut self, repo_id: R) -> &mut Self
        where
            R: Into<String>,
        {
            self.0
                .params
                .insert("searchCriteria.repositoryId", repo_id.into());
            self
        }

        pub fn source_ref_name<R>(&mut self, ref_name: R) -> &mut Self
        where
            R: Into<String>,
        {
            self.0
                .params
                .insert("searchCriteria.sourceRefName", ref_name.into());
            self
        }
        pub fn source_ref_repo_id<R>(&mut self, ref_name: R) -> &mut Self
        where
            R: Into<String>,
        {
            self.0
                .params
                .insert("searchCriteria.sourceRepositoryId", ref_name.into());
            self
        }

        pub fn target_ref_name<R>(&mut self, ref_name: R) -> &mut Self
        where
            R: Into<String>,
        {
            self.0
                .params
                .insert("searchCriteria.targetRefName", ref_name.into());
            self
        }

        pub fn reviewer_id<R>(&mut self, reviewer_id: R) -> &mut Self
        where
            R: Into<String>,
        {
            self.0
                .params
                .insert("searchCriteria.reviewerId", reviewer_id.into());
            self
        }

        pub fn include_links(&mut self, include_links: bool) -> &mut Self {
            self.0
                .params
                .insert("searchCriteria.includeLinks", include_links.to_string());
            self
        }

        pub fn build(&self) -> PullListOptions {
            PullListOptions {
                params: self.0.params.clone(),
            }
        }
    }
}

mod pull_requests_reponse {
    use serde::*;
    #[derive(Debug, Deserialize)]
    pub struct PullRequestsResponse {
        pub value: Vec<Value>,
        pub count: i64,
    }

    #[derive(Debug, Deserialize)]
    pub struct Value {
        pub repository: Repository,
        #[serde(rename = "pullRequestId")]
        pub pull_request_id: u64,
        #[serde(rename = "codeReviewId")]
        pub code_review_id: i64,
        pub status: String,
        #[serde(rename = "createdBy")]
        pub created_by: CreatedBy,
        #[serde(rename = "creationDate")]
        pub creation_date: String,
        pub title: String,
        pub description: String,
        #[serde(rename = "sourceRefName")]
        pub source_ref_name: String,
        #[serde(rename = "targetRefName")]
        pub target_ref_name: String,
        #[serde(rename = "mergeStatus")]
        pub merge_status: String,
        #[serde(rename = "isDraft")]
        pub is_draft: bool,
        #[serde(rename = "mergeId")]
        pub merge_id: String,
        #[serde(rename = "lastMergeSourceCommit")]
        pub last_merge_source_commit: LastMergeSourceCommit,
        #[serde(rename = "lastMergeTargetCommit")]
        pub last_merge_target_commit: LastMergeTargetCommit,
        #[serde(rename = "lastMergeCommit")]
        pub last_merge_commit: LastMergeCommit,
        pub reviewers: Vec<::serde_json::Value>,
        pub url: String,
        #[serde(rename = "supportsIterations")]
        pub supports_iterations: bool,
    }

    #[derive(Debug, Deserialize)]
    pub struct Repository {
        pub id: String,
        pub name: String,
        pub url: String,
        pub project: Project,
    }

    #[derive(Debug, Deserialize)]
    pub struct Project {
        pub id: String,
        pub name: String,
        pub state: String,
        pub visibility: String,
        #[serde(rename = "lastUpdateTime")]
        pub last_update_time: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct CreatedBy {
        #[serde(rename = "displayName")]
        pub display_name: String,
        pub url: String,
        #[serde(rename = "_links")]
        pub links: Links,
        pub id: String,
        #[serde(rename = "uniqueName")]
        pub unique_name: String,
        #[serde(rename = "imageUrl")]
        pub image_url: String,
        pub descriptor: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct Links {
        pub avatar: Avatar,
    }

    #[derive(Debug, Deserialize)]
    pub struct Avatar {
        pub href: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct LastMergeSourceCommit {
        #[serde(rename = "commitId")]
        pub commit_id: String,
        pub url: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct LastMergeTargetCommit {
        #[serde(rename = "commitId")]
        pub commit_id: String,
        pub url: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct LastMergeCommit {
        #[serde(rename = "commitId")]
        pub commit_id: String,
        pub url: String,
    }
}

mod pr_create_options {
    use serde::*;
    #[derive(Debug, Serialize)]
    pub struct PROption {
        #[serde(rename = "sourceRefName")]
        pub source_ref_name: String,
        #[serde(rename = "targetRefName")]
        pub target_ref_name: String,
        pub title: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub reviewers: Option<Vec<Reviewer>>,
    }

    #[derive(Debug, Serialize)]
    pub struct Reviewer {
        pub id: String,
    }
}

mod pull_update_options {
    use super::{MergeStrategy, PullStatus};
    use serde::*;

    #[derive(Debug, Default, Serialize)]
    pub struct PullUpdateOptions {
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        status: Option<PullStatus>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "mergeStrategy")]
        merge_strategy: Option<MergeStrategy>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "lastMergeSourceCommit")]
        source_commit: Option<LastMergeSourceCommit>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "completionOptions")]
        completion_options: Option<CompletionOptions>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "isDraft")]
        draft:Option<bool>
    }

    #[derive(Debug, Default, Serialize, Clone)]
    pub struct LastMergeSourceCommit {
        #[serde(skip_serializing_if = "Option::is_none")]
        commit_id: Option<String>,
    }

    #[derive(Debug, Default, Serialize, Clone)]
    pub struct CompletionOptions {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "bypassPolicy")]
        bypass_policy: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "bypassReason")]
        bypass_reason: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "deleteSourceBranch")]
        delete_source_branch: Option<bool>,
    }

    impl PullUpdateOptions {
        pub fn builder() -> PullUpdateOptionsBuilder {
            PullUpdateOptionsBuilder::default()
        }
    }
    #[derive(Default)]
    pub struct PullUpdateOptionsBuilder(PullUpdateOptions);

    impl PullUpdateOptionsBuilder {
        /// set the title of the pull
        pub fn title<T>(&mut self, title: T) -> &mut Self
        where
            T: Into<String>,
        {
            self.0.title = Some(title.into());
            self
        }
  
        /// set the description of the pull
        pub fn description<B>(&mut self, description: B) -> &mut Self
        where
            B: Into<String>,
        {
            self.0.description = Some(description.into());
            self
        }

        /// set the status of the pull
        pub fn status(&mut self, status: PullStatus) -> &mut Self {
            self.0.status = Some(status.into());
            self
        }

        /// set the merge strategy of the pull
        pub fn merge_strategy(&mut self, merge_strategy: MergeStrategy) -> &mut Self {
            self.0.merge_strategy = Some(merge_strategy.into());
            self
        }

         pub fn draft(&mut self, draft: bool) -> &mut Self {
            self.0.draft = Some(draft);
            self
        }
        // /// set the source commit of the merge
        // pub fn source_commit<C>(&mut self, source_commit: C) -> &mut Self
        // where
        //     C: Into<String>,
        // {
        //     self.0.source_commit = LastMergeSourceCommit {
        //         commit_id: Some(source_commit.into()),
        //     };
        //     self
        // }

        // pub fn bypass_policy(&mut self, bypass_policy: bool) -> &mut Self {
        //     self.0.completion_options.bypass_policy = Some(bypass_policy);
        //     self
        // }

        // /// set the bypass reason of the pull
        // pub fn bypass_reason<B>(&mut self, bypass_reason: B) -> &mut Self
        // where
        //     B: Into<String>,
        // {
        //     self.0.completion_options.bypass_reason = Some(bypass_reason.into());
        //     self
        // }
        // pub fn delete_source_branch(&mut self, delete_source_branch: bool) -> &mut Self {
        //     self.0.completion_options.delete_source_branch = Some(delete_source_branch);
        //     self
        // }

        /// create a new set of pull edit options
        pub fn build(&self) -> PullUpdateOptions {
            PullUpdateOptions {
                title: self.0.title.clone(),
                description: self.0.description.clone(),
                status: self.0.status.clone(),
                draft: self.0.draft.clone(),
                merge_strategy: self.0.merge_strategy.clone(),
                source_commit: self.0.source_commit.clone(),
                completion_options: self.0.completion_options.clone(),
            }
        }
    }
}
/// enum representation of Azure Pull Request Status
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub enum PullStatus {
    Abandoned,
    Active,
    Draft,
    All,
    Completed,
    NotSet,
}

impl fmt::Display for PullStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            PullStatus::Abandoned => "abandoned",
            PullStatus::Active => "active",
            PullStatus::Draft => "draft",
            PullStatus::All => "all",
            PullStatus::Completed => "completed",
            PullStatus::NotSet => "notSet",
        }
        .fmt(f)
    }
}

impl Default for PullStatus {
    fn default() -> PullStatus {
        PullStatus::NotSet
    }
}

/// enum representation of Azure Pull Request Merge Strategies, by default will squash merge
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub enum MergeStrategy {
    NoFastForward,
    Rebase,
    RebaseMerge,
    Squash,
}

impl fmt::Display for MergeStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            MergeStrategy::NoFastForward => "noFastForward",
            MergeStrategy::Rebase => "rebase",
            MergeStrategy::RebaseMerge => "rebaseMerge",
            MergeStrategy::Squash => "squash",
        }
        .fmt(f)
    }
}

// Should the default be squash?
impl Default for MergeStrategy {
    fn default() -> MergeStrategy {
        MergeStrategy::Squash
    }
}
