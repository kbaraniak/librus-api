use serde::Deserialize;


#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Grade {
    pub id: i64,
    pub lesson: GradesRedirect,
    pub subject: GradesRedirect,
    pub student: GradesRedirect,
    pub category: GradesRedirect,
    pub added_by: GradesRedirect,
    pub grade: String,
    pub date: String,
    pub add_date: String,
    pub semester: i64,
    pub is_constituent: bool,
    pub is_semester: bool,
    pub is_semester_proposition: bool,
    pub is_final: bool,
    pub is_final_proposition: bool,
    pub comments: Option<Vec<GradesRedirect>>,
    pub improvement: Option<GradesRedirect>,
    pub resit: Option<GradesRedirect>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GradesRedirect {
    pub id: i32,
    pub url: String,
}



#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GradesUrl {
    pub url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GradesResources {
    #[serde(rename = "Grades\\Averages")]
    pub grades_averages: GradesUrl,
    #[serde(rename = "Grades\\StudentsAverages")]
    pub grades_students_averages: GradesUrl,
    #[serde(rename = "Grades\\CategoriesAverages")]
    pub grades_categories_averages: GradesUrl,
    #[serde(rename = "Grades\\Categories")]
    pub grades_categories: GradesUrl,
    #[serde(rename = "Grades\\Comments")]
    pub grades_comments: GradesUrl,
    #[serde(rename = "Grades\\Scales")]
    pub grades_scales: GradesUrl,
    #[serde(rename = "Grades\\Types")]
    pub grades_types: GradesUrl,
    #[serde(rename = "Grades\\UnpreparednessPerSemesterAndSubject")]
    pub grades_unpreparedness_per_semester_and_subject: GradesUrl,
    #[serde(rename = "..")]
    pub root: GradesUrl,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseGrades{
    pub grades: Vec<Grade>,
    pub resources: GradesResources,
    pub url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GradeColor {
    pub id: i64,
    pub url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GradeCategory {
    pub id: i64,
    pub color: GradeColor,
    pub name: String,
    pub adults_extramural: bool,
    pub adults_daily: bool,
    pub standard: bool,
    pub is_read_only: String,
    pub count_to_the_average: bool,
    pub block_any_grades: bool,
    pub obligation_to_perform: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GradesCategoryResources{
    #[serde(rename = "..")]
    pub root: GradesUrl,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GradeComment {
    pub id: i32,
    pub added_by: GradeDetails,
    pub grade: GradeDetails,
    pub text: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GradeDetails {
    pub id: i64,
    pub url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseGradesCategories{
    pub category: GradeCategory,
    pub resources: GradesCategoryResources,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseGradesComments{
    pub comment: Option<GradeComment>,
    pub resources: GradesCategoryResources,
    pub url: String,

}