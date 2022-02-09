use reqwasm::http::Request;
use serde_derive::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use yew::prelude::*;
#[derive(Serialize, Deserialize, Clone, Debug)]
struct Todo {
    name: String,
    date: String,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=window)]
    fn prompt(s: &str) -> JsValue;
    #[wasm_bindgen(js_namespace=window)]
    fn alert(s: &str);
}
enum Msg {
    AddTodo(Vec<Todo>),
    Init,
    NewTodo,
    Done,
    RmTodo,
}

struct Model {
    todo: Vec<Todo>,
    fetch: bool,
    url: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Model {
        let todo = vec![];
        let url = String::from("http://localhost:8000");
        Model {
            todo,
            fetch: true,
            url,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        let url = self.url.clone();
        let len = self.todo.len();
        match msg {
            Msg::AddTodo(data) => {
                self.todo = data;
                true
            }
            Msg::Init => {
                if self.fetch {
                    _ctx.link()
                        .send_future(async { Msg::AddTodo(init(url).await) });
                    self.fetch = false;
                    true
                } else {
                    false
                }
            }
            Msg::NewTodo => {
                self.fetch = true;
                _ctx.link().send_future(async move {
                    new_todo(url).await;
                    Msg::Done
                });
                false
            }
            Msg::RmTodo => {
                self.fetch = true;
                _ctx.link().send_future(async move {
                    rm_todo(url, len).await;
                    Msg::Done
                });
                false
            }
            Msg::Done => true,
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        // spawn_local(self.init());
        ctx.link().send_message(Msg::Init);
        let todo = self.todo.clone();
        html! {
        <div>
            <div class="head">{"ToDo Handler"}</div>
            <button onclick={ ctx.link().callback(|_|Msg::NewTodo)}>{"New"} </button>
            <button onclick={ctx.link().callback(|_|Msg::RmTodo)}>{"Remove"}</button>
            <div class="todo">
               {
                todo.into_iter().enumerate().map(|(i,todo)|{
                    html!{
                        <div>
                        <div>{i}</div>
                        <div>{format!("{}",todo.name)}</div>
                        <div>{format!("{}",todo.date)}</div>
                        </div>
                    }
                }).collect::<Html>()
            }
            </div>
        </div>
        }
    }
}
async fn init(url: String) -> Vec<Todo> {
    let result: Vec<Todo> = Request::get(format!("{}/init", url).as_str())
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    result
}
async fn new_todo(url: String) {
    let name_js = prompt("Name");
    let mut name = String::new();
    let mut date = String::new();
    if name_js.is_string() {
        name = name_js.as_string().unwrap();
    } else {
        return;
    }
    let date_js = prompt("Date : ");
    if date_js.is_string() {
        date = date_js.as_string().unwrap();
    }
    Request::post(format!("{}/new/{}/{}", url, name, date).as_str())
        .send()
        .await
        .unwrap();
}
async fn rm_todo(url: String, length: usize) {
    let mut index = 0;
    let input = prompt("Enter index of todo to be removed ");
    if input.is_string() {
        index = input.as_string().unwrap().parse().unwrap();
    }
    if index < length {
        Request::post(format!("{}/rm/{}", url, index).as_str())
            .send()
            .await
            .unwrap();
    } else {
        alert("Enter valid index number");
    }
}

fn main() {
    yew::start_app::<Model>();
}
