use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TimetableLesson{
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Url")]
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct TimetableClassroom{
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Url")]
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct TimetableEntry{
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Url")]
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct TimetableLessonSubject{
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Short")]
    pub short: String,
    #[serde(rename = "Url")]
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct TimetableTeacher{
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "FirstName")]
    pub first_name: String,
    #[serde(rename = "LastName")]
    pub last_name: String,
    #[serde(rename = "Url")]
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct TimetableClass{
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Url")]
    pub url: String,
}

#[derive(Deserialize, Debug)]
// #[serde(rename_all = "PascalCase")]
pub struct TimetableDay{
    #[serde(rename = "Lesson")]
    pub lesson: Option<TimetableLesson>,
    // pub classroom: Option<TimetableLesson>,
    // pub date_from: Option<String>,
    // pub date_to: Option<String>,
    // pub lesson_no: Option<String>,
    // pub timetable_entry: Option<TimetableLesson>,
    // pub day_no: Option<String>,
    // pub subject: Option<TimetableLessonSubject>,
    // pub teacher: Option<TimetableTeacher>,
    // pub class: Option<TimetableClassroom>,
    // pub is_substitution_class: Option<bool>,
    // pub is_canceled: Option<bool>,
    // pub substitution_note: Option<serde_json::Value>,
    // pub hour_from: Option<String>,
    // pub hour_to: Option<String>,
    // pub org_classroom: Option<TimetableLesson>,
    // pub org_date: Option<String>,
    // pub org_lesson_no: Option<String>,
    // pub org_lesson: Option<TimetableLesson>,
    // pub org_subject: Option<TimetableLesson>,
    // pub org_teacher: Option<TimetableLesson>,
    // pub org_hour_from: Option<String>,
    // pub org_hour_to: Option<String>,
    // pub substitution_class_url: Option<String>,
    // pub new_date: Option<String>,
    // pub new_lesson_no: Option<String>,
    // pub new_lesson: Option<TimetableLesson>,
    // pub new_subject: Option<TimetableLesson>,
    // pub new_teacher: Option<TimetableLesson>,
    // pub new_hour_from: Option<String>,
    // pub new_hour_to: Option<String>,
    // pub virtual_class: Option<TimetableClassroom>,
    // pub virtual_class_name: Option<String>,
}

#[derive(Debug, Deserialize)]
// #[serde(rename_all = "PascalCase")]
#[serde(rename = "Timetable")]
pub struct Timetable {
    pub timetable: Option<HashMap<String, Vec<Vec<TimetableDay>>>>,
}

#[derive(Debug, Deserialize)]
pub struct TimetablePages{
    #[serde(rename = "Next")]
    pub next: String,
    #[serde(rename = "Prev")]
    pub prev: String,
}

#[derive(Debug, Deserialize)]
pub struct TimetablesUrl{
    #[serde(rename = "Url")]
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct TimetableResources{
    #[serde(rename = "Timetables\\IndividualLearningPath")]
    pub individual_learning_path: TimetablesUrl,
    #[serde(rename = "Timetables\\OneToOneLearningPlan")]
    pub onetoone_learning_plan: TimetablesUrl,
    #[serde(rename = "Timetables\\OtherActivitiesRegister")]
    pub other_activities_register: TimetablesUrl,
    #[serde(rename = "..")]
    pub root: TimetablesUrl,
}


#[derive(Debug, Deserialize)]
pub struct ResponseTimetable {
    #[serde(rename = "Timetable")]
    pub timetable: Timetable,
    #[serde(rename = "Pages")]
    pub pages: TimetablePages,
    #[serde(rename = "Resources")]
    pub resources: TimetableResources,
    #[serde(rename = "Url")]
    pub url: String
}
