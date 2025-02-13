#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "The operation supported by Advisor."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplayInfo {
    #[doc = "The description of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The action that users can perform, based on their permission level."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Service provider: Microsoft Advisor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Resource on which the operation is performed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}
impl OperationDisplayInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The operation supported by Advisor."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationEntity {
    #[doc = "Operation name: {provider}/{resource}/{operation}."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The operation supported by Advisor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplayInfo>,
}
impl OperationEntity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of Advisor operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationEntityListResult {
    #[doc = "The link used to get the next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of operations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationEntity>,
}
impl OperationEntityListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the recommendation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecommendationProperties {
    #[doc = "The category of the recommendation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<recommendation_properties::Category>,
    #[doc = "The business impact of the recommendation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact: Option<recommendation_properties::Impact>,
    #[doc = "The resource type identified by Advisor."]
    #[serde(rename = "impactedField", default, skip_serializing_if = "Option::is_none")]
    pub impacted_field: Option<String>,
    #[doc = "The resource identified by Advisor."]
    #[serde(rename = "impactedValue", default, skip_serializing_if = "Option::is_none")]
    pub impacted_value: Option<String>,
    #[doc = "The most recent time that Advisor checked the validity of the recommendation."]
    #[serde(rename = "lastUpdated", default, skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    #[doc = "The recommendation metadata."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[doc = "The recommendation-type GUID."]
    #[serde(rename = "recommendationTypeId", default, skip_serializing_if = "Option::is_none")]
    pub recommendation_type_id: Option<String>,
    #[doc = "The potential risk of not implementing the recommendation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub risk: Option<recommendation_properties::Risk>,
    #[doc = "A summary of the recommendation."]
    #[serde(rename = "shortDescription", default, skip_serializing_if = "Option::is_none")]
    pub short_description: Option<ShortDescription>,
}
impl RecommendationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod recommendation_properties {
    use super::*;
    #[doc = "The category of the recommendation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Category {
        HighAvailability,
        Security,
        Performance,
        Cost,
    }
    #[doc = "The business impact of the recommendation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Impact {
        High,
        Medium,
        Low,
    }
    #[doc = "The potential risk of not implementing the recommendation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Risk {
        Error,
        Warning,
        None,
    }
}
#[doc = "An Azure resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "The resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The location of the resource. This cannot be changed after the resource is created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The tags of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Advisor Recommendation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceRecommendationBase {
    #[doc = "The fully qualified recommendation ID, for example /subscriptions/subscriptionId/resourceGroups/resourceGroup1/providers/Microsoft.ClassicCompute/virtualMachines/vm1/providers/Microsoft.Advisor/recommendations/recommendationGUID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of recommendation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The properties of the recommendation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RecommendationProperties>,
    #[doc = "The list of snoozed and dismissed rules for the recommendation."]
    #[serde(rename = "suppressionIds", default, skip_serializing_if = "Vec::is_empty")]
    pub suppression_ids: Vec<String>,
    #[doc = "The recommendation type: Microsoft.Advisor/recommendations."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ResourceRecommendationBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of Advisor recommendations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceRecommendationBaseListResult {
    #[doc = "The link used to get the next page of recommendations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of recommendations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ResourceRecommendationBase>,
}
impl ResourceRecommendationBaseListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A summary of the recommendation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ShortDescription {
    #[doc = "The issue or opportunity identified by the recommendation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub problem: Option<String>,
    #[doc = "The remediation action suggested by the recommendation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub solution: Option<String>,
}
impl ShortDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The details of the snoozed or dismissed rule; for example, the duration, name, and GUID associated with the rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SuppressionContract {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The GUID of the suppression."]
    #[serde(rename = "suppressionId", default, skip_serializing_if = "Option::is_none")]
    pub suppression_id: Option<String>,
    #[doc = "The duration for which the suppression is valid."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl: Option<String>,
}
impl SuppressionContract {
    pub fn new() -> Self {
        Self::default()
    }
}
