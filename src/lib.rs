use gloo_net::http::Request;
use serde::Serialize;
use yew::prelude::*;
use serde_json::{Result, Value};
use model::*;
use session::*;

mod model;
mod session;

#[derive(Clone)]
pub struct App {
    session_id: Option<String>,
    current: Option<Movie>,
}

pub enum Msg {
    Start,
    Join,
    Watch(String),
    Skip(String),
    SetCurrent(Movie),
    UpdateState(App),
    Error,
    Nothing,
}

impl App {
    fn start(&mut self, ctx: &Context<Self>) {
        ctx.link().send_future(async {
            let response = Request::get("/api/start")
                .send()
                .await;

            match &response {
                Ok(_) => { log::info!("start request OK"); }
                Err(e) => { log::warn!("{}", e) }
            }

            let session: Session =
                response
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

            Msg::UpdateState(App {
                session_id: Some(session.session_id),
                current: Some(session.movie),
            })
        });
    }

    fn print_an_address() -> String{
        // Some data structure.
        let address = Vote {
            result: "10 Downing Street".to_owned(),
            movie_id: "London".to_owned(),
        };

        // Serialize it to a JSON string.
      serde_json::to_string(&address).unwrap()
    }

    fn vote_watch(&mut self, ctx: &Context<Self>, movie_id: &String) {
        let session_id =  self.session_id.clone().unwrap();
        let movie_id =  movie_id.clone();

        ctx.link().send_future(async {
            let session_id = session_id;
            let movie_id = movie_id;

            let url = "/api/vote/1";


            let vote = Vote {
                result: String::from("true").to_owned(),
                movie_id :  movie_id.clone(),
            };

            let v = serde_json::to_string(&vote).unwrap();

            let response = Request::post(url)
                .header("Content-Type", "application/json")
                .body(v)
                .send()
                .await;

            match &response {
                Ok(_) => { println!("OK") }
                Err(e) => { println!("{}", e) }
            }

            let vote_result: VoteResult =
                response
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

            Msg::SetCurrent(vote_result.movie.clone())
        });
    }

    fn vote_skip(&mut self, ctx: &Context<Self>, movie_id: &String) {
        App::vote_watch(self, ctx, movie_id)
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {
            session_id: None,
            current: None,
        }
    }

    fn update(
        &mut self,
        ctx: &Context<Self>,
        msg: Self::Message,
    ) -> bool {
        match msg {
            Msg::Start => {
                log::info!("Start");
                App::start(self, &ctx);
                true
            }
            Msg::Join => {
                log::info!("Start");
                App::start(self, &ctx);
                false
            }
            Msg::Skip(id) => {
                log::info!("Skip");
                App::vote_skip(self, ctx, &id);
                false
            }
            Msg::Watch(id) => {
                log::info!("Watch");
                App::vote_watch(self, ctx, &id);
                false
            }
            Msg::SetCurrent(movie) => {
                log::info!("SetCurrent");
                self.current = Some(movie.clone());
                true
            }
            Msg::UpdateState(state) => {
                log::info!("UpdateState");
                self.session_id = state.session_id.clone();
                self.current = state.current.clone();
                true
            }
            Msg::Nothing => false,
            Msg::Error => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let movie = self.current.clone().map(|movie| {
            html! {
            <Voter
            movie = {movie}
            on_watch = {ctx.link().callback(|movie_id| Msg::Watch(movie_id))}
            on_skip = {ctx.link().callback(|movie_id| Msg::Skip(movie_id))}
                />
                }
        });

        let poster = self.current.clone().map(|movie| {
            html! { <img src={movie.poster_url}/>
        }
        });

        html! {
<div class="page">
    <div class="card">
        <div class="container">
            <div class="menu">
                <h3>{"movie."}</h3>
                <div class="button" onclick = { ctx.link().callback(|_| Msg::Join) } > {".join"} </div>
                <div class="button" onclick = { ctx.link().callback(|_| Msg::Start) } > {".start"} </div>
            </div>
            {for movie}
        </div>
        <div class="poster">
            {for poster}
        </div>
    </div>
</div>
        }
    }
}

