use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseHomeworks {
    #[serde(rename = "HomeWorks")]
    pub homeworks: Vec<Homework>,
    pub resources: Option<HomeworksResources>,
    pub url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Homework {
    pub id: i64,
    pub content: String,
    pub date: String,
    pub category: HomeworksCategory,
    pub lesson_no: Option<String>,
    pub time_from: String,
    pub time_to: String,
    pub created_by: HomeworksCategory,
    pub class: HomeworksCategory,
    pub subject: HomeworksCategory,
    pub add_date: String,
    pub classroom: Option<HomeworksClassroom>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HomeworksCategory {
    pub id: i64,
    pub url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HomeworksClassroom {
    pub id: i64,
    pub symbol: String,
    pub name: String,
    pub size: i64,
}

#[derive(Debug, Deserialize)]
pub struct HomeworksResources {
    #[serde(rename = "HomeWorks\\Categories")]
    pub homeworks_categories: HomeworksUrl,
    #[serde(rename = "..")]
    pub empty: HomeworksUrl,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HomeworksUrl {
    pub url: String,
}
