use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Account {
    #[serde(rename = "Id")]
    pub id: u32,
    #[serde(rename = "UserId")]
    pub user_id: u32,
    #[serde(rename = "FirstName")]
    pub first_name: String,
    #[serde(rename = "LastName")]
    pub last_name: String,
    #[serde(rename = "Email")]
    pub email: String,
    #[serde(rename = "GroupId")]
    pub group_id: u32,
    #[serde(rename = "IsActive")]
    pub is_active: bool,
    #[serde(rename = "Login")]
    pub login: String,
    #[serde(rename = "IsPremium")]
    pub is_premium: bool,
    #[serde(rename = "IsPremiumDemo")]
    pub is_premium_demo: bool,
    #[serde(rename = "ExpiredPremiumDate")]
    pub expired_premium_date: u64,
    #[serde(rename = "PremiumAddons")]
    pub premium_addons: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct User {
    #[serde(rename = "FirstName")]
    pub first_name: String,
    #[serde(rename = "LastName")]
    pub last_name: String,
}

#[derive(Debug, Deserialize)]
pub struct Class {
    #[serde(rename = "Id")]
    pub id: u32,
    #[serde(rename = "Url")]
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Resource {
    #[serde(rename = "Url")]
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Resources {
    #[serde(rename = "..")]
    pub url: Resource,
}

#[derive(Debug, Deserialize)]
pub struct Me {
    #[serde(rename = "Account")]
    pub account: Account,
    #[serde(rename = "Refresh")]
    pub refresh: u32,
    #[serde(rename = "User")]
    pub user: User,
    #[serde(rename = "Class")]
    pub class: Class,
}

#[derive(Debug, Deserialize)]
pub struct ResponseMe {
    #[serde(rename = "Me")]
    pub me: Me,
    #[serde(rename = "Resources")]
    pub resources: Resources,
    #[serde(rename = "Url")]
    pub url: String,
}