use gloo_net::Error;
use gloo_net::http::{Request, Response};
use log::log;
use yew::prelude::*;

use model::*;
use session::*;

mod model;
mod session;

pub struct App {
    session_id: Option<String>,
    user_id: Option<String>,
    current: Option<Movie>,
}

pub enum Msg {
    Start,
    Watch(String),
    Skip(String),
    SetCurrent(Movie),
    Error,
}

impl App {
    fn start(ctx: &Context<Self>) {
        ctx.link().send_future(async {
            let response = Request::get("/api/start")
                .send()
                .await;

            match &response {
                Ok(_) => { log::info!("start request OK"); }
                Err(e) => { log::warn!("{}", e) }
            }

            let fetched_movie: Movie =
                response
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

            Msg::SetCurrent(fetched_movie)
        });
    }

    fn get_data(ctx: &Context<Self>) {
        ctx.link().send_future(async {
            let response = Request::get("/api/movie")
                .send()
                .await;

            match &response {
                Ok(_) => { println!("OK") }
                Err(e) => { println!("{}", e) }
            }

            let fetched_movie: Movie =
                response
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

            Msg::SetCurrent(fetched_movie)
        });
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {

        Self {
            user_id: None,
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
                App::start(&ctx);
                false
            }
            Msg::Skip(id) | Msg::Watch(id) => {
                App::get_data(ctx);
                false
            }
            Msg::SetCurrent(movie) => {
                self.current = Some(movie.clone());
                true
            }
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
                <div class="button" onclick = { ctx.link().callback(|_| Msg::Watch("".into())) } > {".join"} </div>
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

