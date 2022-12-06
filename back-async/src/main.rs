#[macro_use]
extern crate rocket;
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::Header,
    tokio::{
        fs::{File, OpenOptions},
        io::{AsyncReadExt, AsyncWriteExt},
    },
    Request, Response,
};
use serde_derive::{Deserialize, Serialize};
use serde_json::to_vec_pretty;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Todo {
    name: String,
    date: String,
}
async fn read_json() -> Option<Vec<Todo>> {
    let f = OpenOptions::new()
        .write(true)
        .read(true)
        .open("./data.json")
        .await;
    if let Ok(mut file) = f {
        let mut buff = Vec::<u8>::new();
        file.read_to_end(&mut buff).await.expect("msg");
        let result = String::from_utf8(buff).expect("msg");
        let jresult: Result<Vec<Todo>, _> = serde_json::from_str(&result);
        let mut fresult = Vec::<Todo>::new();
        if jresult.is_ok() {
            fresult = jresult.unwrap();
        } else {
        }
        Some(fresult)
    } else {
        let mut file = File::create("./data.json")
            .await
            .expect("Failed to create FIle:");
        file.write(b"[]").await.expect("filljson");
        file.flush().await.unwrap();
        None
    }
}
#[get("/init")]
async fn init_app() -> String {
    if let Some(result) = read_json().await {
        to_string(&result).unwrap()
    } else {
        to_string(&read_json().await.unwrap()).unwrap()
    }
}
#[post("/new/<name>/<date>")]
async fn new_todo(name: String, date: String) {
    let mut data = read_json().await.unwrap_or_default();
    data.append(&mut vec![Todo { name, date }]);
    write_json(data).await;
}
#[post("/rm/<index>")]
async fn remove_todo(index: usize) {
    let mut data = read_json().await.unwrap();
    data.remove(index);
    write_json(data).await;
}

async fn write_json(data: Vec<Todo>) {
    let mut f = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("./data.json")
        .await
        .unwrap();
    f.write(&to_vec_pretty(&data).unwrap()).await.unwrap();
    f.flush().await.unwrap();
}

pub struct CORS;
#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    fn on_response<'r, 'life0, 'life1, 'life2, 'async_trait>(
        &'life0 self,
        _req: &'r Request<'life1>,
        _res: &'life2 mut Response<'r>,
    ) -> core::pin::Pin<
        Box<dyn core::future::Future<Output = ()> + core::marker::Send + 'async_trait>,
    >
    where
        'r: 'async_trait,
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        _res.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        _res.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        _res.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        _res.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        Box::pin(async move {
            let __self = self;
            let _req = _req;
            let _res = _res;
        })
    }
}
#[get("/")]
async fn index() -> String {
    "Welcome ToDo app  \n Use /init to get data \n /new/name/date to add todo /n /rm/index to remove a todo".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, new_todo, init_app, remove_todo])
        .attach(CORS)
}
