use gloo_net::Error;
use gloo_net::http::{Request, Response};
use yew::prelude::*;

use movie::*;
use session::*;

mod movie;
mod session;

pub struct App {
    current: Movie,
}

pub enum Msg {
    Start,
    Watch(String),
    Skip(String),
    SetCurrent(Movie),
    Error,
}

impl App {
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
            current: Movie {
                id: "1".into(),
                title: "Inception".into(),
                genres: vec!["Action".into(), "Adventure".into(), "Sci-Fi".into()],
                description: "A thief who steals corporate secrets through the use of dream-sharing technology is given the inverse task of planting an idea into the mind of a C.E.O., but his tragic past may doom the project and his team to disaster.".into(),
                poster_url: "https://cdn.shopify.com/s/files/1/0037/8008/3782/products/inception_advance_SD18120_B_2_framed1_57a8f726-e4da-4a60-877b-95b210b8fc91-367857.jpg?v=1611688027".into(),
            },
        }
    }

    fn update(
        &mut self,
        _ctx: &Context<Self>,
        msg: Self::Message,
    ) -> bool {
        match msg {
            Msg::Start => { false }
            Msg::Skip(id) | Msg::Watch(id) => {
                App::get_data(_ctx);
                false
            }
            Msg::SetCurrent(movie) => {
                self.current = movie.clone();
                true
            }
            Msg::Error => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let movie = self.current.clone();

        html! {
<div class="page">
    <div class="card">
        <div class="container">
            <div class="menu">
                <h3>{"movie."}</h3>
            </div>
            <form id="form">
                <input id="user_id" type="text" placeholder="USER"/>
                <input id="session_id" type="text" placeholder="SESSION"/>
            </form>
            <Voter
                movie = {movie.clone()}
                on_watch = {ctx.link().callback(|movie_id| Msg::Watch(movie_id))}
                on_skip = {ctx.link().callback(|movie_id| Msg::Skip(movie_id))}
            />
        </div>
        <div class="poster">
            <img src={movie.poster_url}/>
        </div>
    </div>
</div>
        }
    }
}

