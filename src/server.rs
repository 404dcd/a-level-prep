use self::rusqlite::params;
use rocket::fairing::AdHoc;
use rocket::form::Form;
use rocket::fs::{relative, FileServer};
use rocket::{Build, Rocket};
use rocket_dyn_templates::Template;
use rocket_sync_db_pools::{database, rusqlite};

use std::collections::HashMap;

#[derive(Debug, FromForm, Clone)]
struct Details {
    input_first: String,
    input_last: String,
    input_address: String,
    input_address2: String,
    input_city: String,
    input_post: String,
}

#[database("sqlite_logs")]
struct Db(rusqlite::Connection);

#[get("/list")]
async fn list_users(db: Db) -> String {
    let data = db
        .run(|conn| {
            conn.prepare("SELECT * FROM students")?
                .query_map(params![], |row| {
                    Ok((
                        row.get(0)?,
                        row.get(1)?,
                        row.get(2)?,
                        row.get(3)?,
                        row.get(4)?,
                        row.get(5)?,
                        row.get(6)?,
                    ))
                })?
                .collect::<Result<Vec<(i32, String, String, String, String, String, String)>, _>>()
        })
        .await
        .unwrap();

    let mut ret = String::from("id|first name|last name|address|address 2|city|post code\n");
    for row in data {
        ret.push_str(
            format!(
                "{}|{}|{}|{}|{}|{}|{}\n",
                row.0, row.1, row.2, row.3, row.4, row.5, row.6
            )
            .as_str(),
        )
    }
    ret
}

#[get("/")]
fn index() -> Template {
    let c: HashMap<bool, bool> = HashMap::new();
    Template::render("index", &c)
}

#[post("/", data = "<data>")]
async fn save(db: Db, data: Form<Details>) -> &'static str {
    let p1 = data.input_first.clone().replace("|", "");
    let p2 = data.input_last.clone().replace("|", "");
    let p3 = data.input_address.clone().replace("|", "");
    let p4 = data.input_address2.clone().replace("|", "");
    let p5 = data.input_city.clone().replace("|", "");
    let p6 = data.input_post.clone().replace("|", "");

    if let Ok(_) = db.run(move |conn| conn.execute("INSERT INTO students (first, last, address, address2, city, post) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![p1, p2, p3, p4, p5, p6])).await {
        "Success"
    } else {
        "Failed to update database"
    }
}

async fn init_db(rocket: Rocket<Build>) -> Rocket<Build> {
    Db::get_one(&rocket)
        .await
        .unwrap()
        .run(|conn| {
            conn.execute(
                r#"
                CREATE TABLE students (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    first TEXT NOT NULL,
                    last TEXT NOT NULL,
                    address TEXT NOT NULL,
                    address2 TEXT NOT NULL,
                    city TEXT NOT NULL,
                    post TEXT NOT NULL
                );"#,
                params![],
            )
        })
        .await
        .unwrap();

    rocket
}

#[launch]
pub fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .attach(Db::fairing())
        .attach(AdHoc::on_ignite("Rusqlite Init", init_db))
        .mount("/", routes![index, save,])
        .mount("/api", routes![list_users])
        .mount("/static", FileServer::from(relative!("static")))
}
