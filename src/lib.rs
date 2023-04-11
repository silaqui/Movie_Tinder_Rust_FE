use gloo_net::http::Request;
use yew::prelude::*;

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
    SetCurrent(Option<Movie>),
    UpdateState(App),
    Error,
    Nothing,
}

impl App {
    fn start(&mut self, ctx: &Context<Self>) {
        ctx.link().send_future(async {
            let session: SessionStateDTO =
                Request::get("/api/start")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

            Msg::UpdateState(App {
                session_id: Some(session.session_id),
                current: session.next_movie,
            })
        });
    }

    fn join(&mut self, ctx: &Context<Self>) {
        ctx.link().send_future(async {
            let session: SessionStateDTO =
                Request::get("/api/join/1")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

            Msg::UpdateState(App {
                session_id: Some(session.session_id),
                current: session.next_movie,
            })
        });
    }

    fn vote_watch(&mut self, ctx: &Context<Self>, movie_id: &String) {
        self.vote(ctx, &movie_id, VoteResult::WATCH)
    }

    fn vote_skip(&mut self, ctx: &Context<Self>, movie_id: &String) {
        self.vote(ctx, &movie_id, VoteResult::SKIP)
    }

    fn vote(&mut self, ctx: &Context<Self>, movie_id: &String, result: VoteResult) {
        {
            let movie_id = movie_id.clone();
            let session_id = self.session_id.clone().unwrap();

            ctx.link().send_future(async {
                let mut url = "/api/vote/".to_string();
                let session_id = session_id;

                url.push_str(session_id.as_str());

                let vote_json = serde_json::to_string(&VoteDTO {
                    result,
                    movie_id,
                }).unwrap();

                let vote_result: SessionStateDTO =
                    Request::post(url.as_str())
                        .header("Content-Type", "application/json")
                        .body(vote_json)
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();

                Msg::SetCurrent(vote_result.next_movie.clone())
            });
        }
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
                App::join(self, &ctx);
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
                self.current = movie.clone();
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

