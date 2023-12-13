use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct User {
    pub id: i64,
    pub account_id: String,
    pub first_name: String,
    pub last_name: String,
    pub class: Option<UserClass>,
    pub unit: Option<UserUnit>,
    pub class_register_number: Option<i64>,
    pub is_employee: bool,
    pub group_id: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserClass {
    pub id: i64,
    pub url: String,
    #[serde(rename = "UUID")]
    pub uuid: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserUnit {
    pub id: i64,
    pub url: String,
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserUrl {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct UserResources {
    #[serde(rename = "Users\\IndividualEducationPeriods")]
    pub users_individual_education_periods: UserUrl,
    #[serde(rename = "Users\\CrossedOutStudents")]
    pub users_crossed_out_students: UserUrl,
    #[serde(rename = "..")]
    pub root: UserUrl,
}



#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseUser {
    pub user: Option<User>,
    pub resources: UserResources,
    pub url: String,
}