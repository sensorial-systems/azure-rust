use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkItems {
    pub work_items: Vec<WorkItemRef>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkItemRef {
    pub id: usize,
    pub url: String
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkItem {
    pub id: usize,
    pub fields: WorkItemFields,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkItemFields {
    #[serde(rename = "System.Title")]
    pub title: String,
    #[serde(rename = "Custom.Jira")]
    pub jira: Option<String>,
}