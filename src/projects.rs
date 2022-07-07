//! Projects interface
use std::collections::HashMap;

use url::form_urlencoded;

use crate::{AzureClient, Future};
pub use new_project_options::{ProjectsOptions, ProjectsOptionsBuilder};
pub use new_project_response::ProjectStatus;
pub use project_list_response::ProjectsResponse;
pub use project_response::ProjectResponse;

pub struct Projects {
    ops: AzureClient,
}

impl Projects {
    #[doc(hidden)]
    pub fn new(ops: AzureClient) -> Self {
        Self { ops }
    }

    /// Create a new project
    pub fn create(&self, project: &ProjectsOptions) -> Future<ProjectStatus> {
        self.ops.post(&self.path(""), json!(project))
    }

    fn path(&self, more: &str) -> String {
        format!("/{}/{}/_apis/projects", self.ops.org, more)
    }

    /// List existing projects
    pub fn list(&self, options: &ProjectOptions) -> Future<ProjectsResponse> {
        let mut uri = vec![self.path("")];
        if let Some(query) = options.serialize() {
            uri.push(query);
        }

        // FIXME: It seems incorrect. Only one ? it used in a URL to start a query and they should be separated with &.
        self.ops.get(&uri.join("?"))
    }
}
mod new_project_response {
    use serde::*;
    #[derive(Debug, Default, Deserialize)]
    pub struct ProjectStatus {
        pub id: String,
        pub status: String,
        pub url: String,
    }
}

mod new_project_options {
    use serde::*;
    // #[serde(skip_serializing_if = "Option::is_none")]

    #[derive(Debug, Default, Serialize)]
    pub struct ProjectsOptions {
        pub name: String,
        pub description: String,
        pub capabilities: Capabilities,
    }
    #[derive(Debug, Default, Serialize)]
    pub struct Capabilities {
        pub versioncontrol: Versioncontrol,
        #[serde(rename = "processTemplate")]
        pub process_template: ProcessTemplate,
    }
    #[derive(Debug, Default, Serialize)]
    pub struct Versioncontrol {
        #[serde(rename = "sourceControlType")]
        pub source_control_type: String,
    }
    #[derive(Debug, Default, Serialize)]
    pub struct ProcessTemplate {
        #[serde(rename = "templateTypeId")]
        pub template_type_id: String,
    }

    pub struct ProjectsOptionsBuilder(ProjectsOptions);

    impl ProjectsOptionsBuilder {
        pub(crate) fn new<N>(name: N) -> Self
        where
            N: Into<String>,
        {
            ProjectsOptionsBuilder(ProjectsOptions {
                name: name.into(),
                ..Default::default()
            })
        }

        pub fn description<D>(&mut self, description: D) -> &mut Self
        where
            D: Into<String>,
        {
            self.0.description = description.into();
            self
        }

        pub fn source_control_type<H>(&mut self, source_control_type: H) -> &mut Self
        where
            H: Into<String>,
        {
            self.0.capabilities.versioncontrol.source_control_type = source_control_type.into();
            self
        }

        pub fn template_type_id(&mut self, template_type_id: String) -> &mut Self {
            self.0.capabilities.process_template.template_type_id = template_type_id;
            self
        }

        pub fn build(&self) -> ProjectsOptions {
            ProjectsOptions::new(
                self.0.name.as_str(),
                self.0.description.as_str(),
                self.0
                    .capabilities
                    .versioncontrol
                    .source_control_type
                    .clone(),
                self.0
                    .capabilities
                    .process_template
                    .template_type_id
                    .clone(),
            )
        }
    }

    impl ProjectsOptions {
        #[allow(clippy::too_many_arguments)] // exempted
        pub fn new<N, D, H, E>(
            name: N,
            description: D,
            source_control_type: H,
            template_type_id: E,
        ) -> Self
        where
            N: Into<String>,
            D: Into<String>,
            H: Into<String>,
            E: Into<String>,
        {
            ProjectsOptions {
                name: name.into(),
                description: description.into(),
                capabilities: Capabilities {
                    versioncontrol: Versioncontrol {
                        source_control_type: source_control_type.into(),
                    },
                    process_template: ProcessTemplate {
                        template_type_id: template_type_id.into(),
                    },
                },
            }
        }

        pub fn builder<N: Into<String>>(name: N) -> ProjectsOptionsBuilder {
            ProjectsOptionsBuilder::new(name)
        }
    }
}

mod project_list_response {
    use serde::Deserialize;
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ProjectsResponse {
        pub value: Vec<Value>,
        pub count: i64,
    }
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Value {
        pub id: String,
        pub name: String,
        pub url: String,
        pub state: String,
        pub revision: i64,
        pub visibility: String,
        pub last_update_time: String,
    }
}

#[derive(Default)]
pub struct ProjectOptions {
    params: HashMap<&'static str, String>,
}

impl ProjectOptions {
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

pub struct Project {
    ops: AzureClient,
    project: String,
}

impl Project {
    #[doc(hidden)]
    pub fn new<P>(ops: AzureClient, project: P) -> Self
    where
        P: Into<String>,
    {
        Project {
            ops: ops,
            project: project.into(),
        }
    }

    pub fn get(&self) -> Future<ProjectResponse> {
        self.ops.get(&self.path(""))
    }

    pub fn delete(&self) -> Future<ProjectStatus> {
        self.ops.delete(&self.path(""))
    }

    // GET https://dev.azure.com/{organization}/_apis/projects/{projectId}?api-version=5.1
    fn path(&self, more: &str) -> String {
        format!("/{}/_apis/projects/{}{}", self.ops.org, self.project, more)
    }
}

mod project_response {
    use serde::Deserialize;
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ProjectResponse {
        pub id: String,
        pub name: String,
        pub url: String,
        pub state: String,
        pub revision: i64,
        #[serde(rename = "_links")]
        pub links: Links,
        pub visibility: String,
        #[serde(rename = "defaultTeam")]
        pub default_team: DefaultTeam,
        #[serde(rename = "lastUpdateTime")]
        pub last_update_time: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct Links {
        #[serde(rename = "self")]
        pub self_field: Href,
        pub collection: Collection,
        pub web: Web,
    }

    #[derive(Debug, Deserialize)]
    pub struct Href {
        pub href: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct Collection {
        pub href: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct Web {
        pub href: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct DefaultTeam {
        pub id: String,
        pub name: String,
        pub url: String,
    }
}
