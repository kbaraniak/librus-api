use librus_api::hello;
use librus_api::test_connect;
use librus_api::test_cookie;
use librus_api::get_token;
use librus_api::UserAuthenticated;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    hello();
    test_connect().await;
    test_cookie().await;
    let api = get_token("login", "password").await;
    // api.me().await;
    // Unavailable: api.timetables().await;
    // api.grades().await;
    // api.grade_category(1219422).await;
    // api.lesson(4535499).await;
    // api.lesson_subject(21390).await;
    // api.grade_comment(21068).await;
    // api.user(1793540).await;

    /* Grades API */
    // let all_grades = api.grades().await.grades;
    // for (index, _) in all_grades.iter().enumerate(){
    //     let lesson_grade = &all_grades[index].subject.id;
    //     let grade = &all_grades[index].grade;
    //     let add_date = &all_grades[index].date;
    //     let lesson_subject = api.lesson_subject(*lesson_grade).await;
    //     let lesson_name = &lesson_subject.subject.as_ref().unwrap().name;
    //     let lesson_short = &lesson_subject.subject.as_ref().unwrap().short;
    //     let category_id = &all_grades[index].category.id;
    //     let grade_category = api.grade_category(*category_id).await.category.name;
    //     let teacher_id = &all_grades[index].added_by.id;
    //     let grade_teacher = api.user(*teacher_id).await.user.unwrap();
    //     let teacher_name = grade_teacher.first_name + " " + &grade_teacher.last_name;
    //     println!("\nLesson Name: {:?}", lesson_name);
    //     println!("Short Name: {:?}", lesson_short);
    //     println!("Added By: {:?}", teacher_name);
    //     println!("Grade: {}\nAdd Date: {}", grade, add_date);
    //     println!("Category: {:?}", grade_category);
    // }
    /* Attendances API */
    async fn attendances_types(api: &mut UserAuthenticated) -> HashMap<i32, String>{
        let attendance = api.attendances_type().await.types;
        let mut val = HashMap::new();
        for (index, _) in attendance.iter().enumerate(){
            val.insert(attendance[index].id, attendance[index].name.clone());
        }
        val
    }
    // let all_attendances = api.attendances().await.attendances;
    // for (index, _) in all_attendances.iter().enumerate(){
    //         let lesson_id = &all_attendances[index].lesson.id;
    //         let lesson_subject_id = api.lesson(*lesson_id).await.lesson.subject.id;
    //         let lesson_subject = api.lesson_subject(lesson_subject_id).await;
    //         let lesson_name = &lesson_subject.subject.as_ref().unwrap().name;
    //         let attendance_date = &all_attendances[index].date;
    //         let lesson_num = &all_attendances[index].lesson_no;
    //         let attendance_type = &all_attendances[index].attendance_type.id;
    //         let attendance_name = &attendances_types(&mut api).await[attendance_type];
    //         println!("\nLesson Name: {:?}", lesson_name);
    //         println!("Date: {:?}", attendance_date);
    //         println!("Lesson Num: {:?}", lesson_num);
    //         println!("Type: {:?}", attendance_name);
    // }

    /* Events API */
    // let all_events = api.homeworks().await.homeworks;
    // for (index, _) in all_events.iter().enumerate(){
    //     let event_name = &all_events[index].content;
    //     let event_date = &all_events[index].date;
    //     let event_lesson = &all_events[index].lesson_no;
    //     let event_time_from = &all_events[index].time_from;
    //     let event_time_to = &all_events[index].time_to;
    //     let event_time = event_time_from.to_owned() + " - " + event_time_to;
    //     let teacher_id = &all_events[index].created_by.id;
    //     let event_teacher = api.user((*teacher_id).try_into().unwrap()).await.user.unwrap();
    //     let teacher_name = event_teacher.first_name + " " + &event_teacher.last_name;
    //     println!("\nEvent Name: {:?}", event_name);
    //     println!("Date: {:?}", event_date);
    //     println!("Lesson: {:?}", event_lesson);
    //     println!("Time: {:?}", event_time);
    //     println!("Teacher: {:?}", teacher_name);
    // }

    
    
}
