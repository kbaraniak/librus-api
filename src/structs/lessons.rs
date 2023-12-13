use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Lesson {
    pub id: i32,
    pub teacher: LessonClass,
    pub subject: LessonClass,
    pub class: LessonClass,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LessonClass {
    pub id: i32,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct LessonResources {
    #[serde(rename = "..")]
    pub root: LessonUrl,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LessonUrl {
    pub url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseLesson {
    pub lesson: Lesson,
    pub resources: LessonResources,
    pub url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LessonSubject {
    pub id: i32,
    pub name: String,
    #[serde(rename = "No")]
    pub num: i32,
    pub short: String,
    pub is_extra_curricular: Option<bool>,
    pub is_block_lesson: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseLessonSubject {
    pub subject: Option<LessonSubject>,
    pub resources: LessonResources,
    pub url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Attendance {
    pub id: AttendanceId,
    pub lesson: AttendanceAddedBy,
    pub student: AttendanceAddedBy,
    pub date: String,
    pub add_date: String,
    pub lesson_no: i32,
    pub semester: i32,
    #[serde(rename = "Type")]
    pub attendance_type: AttendanceAddedBy,
    pub added_by: AttendanceAddedBy,
    pub trip: Option<AttendanceAddedBy>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AttendanceAddedBy {
    pub id: i32,
    pub url: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum AttendanceId {
    Integer(i32),
    String(String),
}

#[derive(Debug, Deserialize)]
pub struct AttendanceResources {
    #[serde(rename = "Attendances\\Types")]
    pub attendances_types: LessonUrl,
    #[serde(rename = "Attendances\\LessonsStatistics")]
    pub attendances_lessons_statistics: LessonUrl,
    #[serde(rename = "Attendances\\FilledByTeacher")]
    pub attendances_filled_by_teacher: LessonUrl,
    #[serde(rename = "..")]
    pub empty: LessonUrl,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseAttendances {
    pub attendances: Vec<Attendance>,
    pub resources: AttendanceResources,
    pub url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AttendanceType {
    pub id: i32,
    pub name: String,
    pub short: String,
    pub standard: bool,
    #[serde(rename = "ColorRGB")]
    pub color_rgb: Option<String>,
    pub is_presence_kind: bool,
    pub order: i32,
    pub identifier: String,
    pub standard_type: Option<AttendanceColor>,
    pub color: Option<AttendanceColor>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AttendanceColor {
    pub id: i32,
    pub url: String,
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseAttendancesType {
    pub types: Vec<AttendanceType>,
    pub resources: LessonResources,
    pub url: String,
}