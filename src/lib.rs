use reqwest::Client;
use serde_json;
use serde::__private::fmt::Debug;
use serde::{Serialize, Deserialize};
mod structs;

use crate::structs::{me::*,grades::*,lessons::*,users::*,events::*};


#[derive(Serialize, Deserialize, Debug)]
struct LoginRequest {
    action: String,
    login: String,
    pass: String,
}

pub struct UserAuthenticated{
    pub client: Client,
    pub status: bool,
}

impl UserAuthenticated{
    // let client = 2; - how to set default variable in implementations (impl) Rust
    pub fn hello(self){
        println!("Hello Impl");
        println!("{}", self.status)
    }
    async fn __get_api(&self, endpoint: &str) -> String{
        let response = self.client.get("https://synergia.librus.pl/gateway/api/2.0/".to_owned() + &endpoint)
            .header("Content-Type", "application/json")
            .send()
            .await
            .unwrap();
        let result = response.text().await.unwrap();
        result
    }
    pub async fn me(&self) -> ResponseMe{
        let json_string = self.__get_api("Me").await;
        let me: ResponseMe = serde_json::from_str(&json_string).unwrap();
        me
    }
    pub async fn grades(&self) -> ResponseGrades{
        let json_string = self.__get_api("Grades").await;
        let grades: ResponseGrades = serde_json::from_str(&json_string).unwrap();
        grades
    }

    pub async fn grade_category(&self, num: i32) -> ResponseGradesCategories{
        let json_string = self.__get_api(&("Grades/Categories/".to_owned() + &num.to_string())).await;
        let category: ResponseGradesCategories = serde_json::from_str(&json_string).unwrap();
        category
    }
    pub async fn grade_comment(&self, num: i32) -> ResponseGradesComments{
        let json_string = self.__get_api(&("Grades/Comments/".to_owned() + &num.to_string())).await;
        let comment: ResponseGradesComments = serde_json::from_str(&json_string).unwrap();
        comment
    }
    pub async fn lesson(&self, num: i32) -> ResponseLesson{
        let json_string = self.__get_api(&("Lessons/".to_owned() + &num.to_string())).await;
        let lesson: ResponseLesson = serde_json::from_str(&json_string).unwrap();
        lesson
    }
    pub async fn lesson_subject(&self, num: i32) -> ResponseLessonSubject{
        let json_string = self.__get_api(&("Subjects/".to_owned() + &num.to_string())).await;
        let lesson: ResponseLessonSubject = serde_json::from_str(&json_string).unwrap();
        lesson
    }
    pub async fn attendances(&self) -> ResponseAttendances{
        let json_string = self.__get_api(&("Attendances/".to_owned())).await;
        let attendance: ResponseAttendances = serde_json::from_str(&json_string).unwrap();
        attendance
    }
    pub async fn attendances_type(&self) -> ResponseAttendancesType{
        let json_string = self.__get_api(&("Attendances/Types/".to_owned())).await;
        let attendance: ResponseAttendancesType = serde_json::from_str(&json_string).unwrap();
        attendance
    }
    /* Unavailable for this time, add in the next updates */
    // pub async fn timetables(&self) -> ResponseTimetables{
    //     let json_string = self.__get_api(&("Timetables/".to_owned())).await;
    //     let timetables: ResponseTimetables = serde_json::from_str(&json_string).unwrap();
    //     timetables
    // }
    
    pub async fn homeworks(&self) -> ResponseHomeworks{
        let json_string = self.__get_api(&("HomeWorks/".to_owned())).await;
        let homeworks: ResponseHomeworks = serde_json::from_str(&json_string).unwrap();
        homeworks
    }
    pub async fn user(&self, num: i32) -> ResponseUser{
        let json_string = self.__get_api(&("Users/".to_owned() + &num.to_string())).await;
        let user: ResponseUser = serde_json::from_str(&json_string).unwrap();
        user
    }
    pub async fn user_me(&self) -> ResponseUser{
        let json_string = self.__get_api("Users").await;
        let user: ResponseUser = serde_json::from_str(&json_string).unwrap();
        user
    }

}

pub fn hello() -> String {
    println!("Welcome");
    "Hello\n".to_string()
}

pub async fn test_connect(){
    let _result = reqwest::get("https://example.com").await;
    match _result {
        Ok(_v) => println!("[Connect] Test passed"),
        Err(_e) => println!("[Connect] Network Error"),
    };
}



pub async fn test_cookie(){
    let resp = reqwest::get("http://webtest.5v.pl/cookie/").await.unwrap();

    // Get the headers
    let header = resp.headers();
    let cookie_header = easycookie::set_header(header.clone()).await;
    let first_cookie = cookie_header.get_cookie("random").await;
    let second_cookie = cookie_header.get_cookie("random2").await;
    println!("First Cookie Name: {:?} Value {:?}", "random", first_cookie.get_value());
    println!("Second Cookie Name: {:?} Value {:?}", "random2", second_cookie.get_value());

}
pub async fn get_token(_login: &str, _pass: &str) -> UserAuthenticated{
    let testauth_url = "https://api.librus.pl/OAuth/Authorization?client_id=46&response_type=code&scope=mydata";
    let auth_url = "https://api.librus.pl/OAuth/Authorization?client_id=46";
    let grand_url = "https://api.librus.pl/OAuth/Authorization/Grant?client_id=46";
    let login_request = LoginRequest {
        action: "login".to_string(),
        login: _login.to_string(),
        pass: _pass.to_string(),
    };
    let client = Client::builder().cookie_store(true).build().unwrap();
    // First Authentication
    let _ = client.get(testauth_url).send().await;

    // Second Authentication
    let _response = client.post(auth_url)
        .form(&login_request)
        .send()
        .await.unwrap();

    println!("{}", _response.text().await.unwrap());

    // Authenticate User from Cookie
    let _ = client.get(grand_url).send().await;
    let mut _status = false;
    let token_info = client.get("https://synergia.librus.pl/gateway/api/2.0/Auth/TokenInfo/").send().await.unwrap();
    if token_info.status() == 200{
        // println!("Token Info: {}", token_info.text().await.unwrap());
        let _status = true;
    }
    else{
        println!("[Librus API] Failed Authentication");
        let _status = false;
    }
    UserAuthenticated{client, status: _status}

}


#[test]
fn test() {
    hello();
}