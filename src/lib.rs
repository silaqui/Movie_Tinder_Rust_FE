use yew::prelude::*;

use movie::*;

mod movie;

pub enum Msg {
    Watch,
    Skip,
    Error,
}

pub struct App {
    movies: Vec<Movie>,
    current: Movie,
}

impl App {
    fn my_function(&mut self, ctx: &Context<Self>) {
        self.update(ctx, Msg::Skip);
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {
            movies: vec![
                Movie {
                    id: "1".into(),
                    title: "Inception".into(),
                    genres: vec!["Action".into(), "Adventure".into(), "Sci-Fi".into()],
                    description: "A thief who steals corporate secrets through the use of dream-sharing technology is given the inverse task of planting an idea into the mind of a C.E.O., but his tragic past may doom the project and his team to disaster.".into(),
                    poster_url: "https://cdn.shopify.com/s/files/1/0037/8008/3782/products/inception_advance_SD18120_B_2_framed1_57a8f726-e4da-4a60-877b-95b210b8fc91-367857.jpg?v=1611688027".into(),
                },
                Movie {
                    id: "2".into(),
                    title: "The Shawshank Redemption".into(),
                    genres: vec!["Drama".into()],
                    description: "Over the course of several years, two convicts form a friendship, seeking consolation and, eventually, redemption through basic compassion.".into(),
                    poster_url: "https://i.etsystatic.com/16821137/r/il/c8b3e3/1879586236/il_570xN.1879586236_kdtm.jpg".into(),
                },
            ],
            current: Movie {
                id: "2".into(),
                title: "The Shawshank Redemption".into(),
                genres: vec!["Drama".into()],
                description: "Over the course of several years, two convicts form a friendship, seeking consolation and, eventually, redemption through basic compassion.".into(),
                poster_url: "https://i.etsystatic.com/16821137/r/il/c8b3e3/1879586236/il_570xN.1879586236_kdtm.jpg".into(),
            },
        }
    }

    fn update(
        &mut self,
        _ctx: &Context<Self>,
        msg: Self::Message,
    ) -> bool {
        match msg {
            Msg::Skip | Msg::Watch => {
                let movies = self.movies.clone();
                let current_id = self.current.clone().id;
                for m in movies {
                    if m.id != current_id {
                        self.current = m.clone();
                    }
                }
                true
            }
            Msg::Error => todo!(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let current = self.current.clone();
        let genres = current.genres.join("|");

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
            <div class="content">
                <div class="text">
                    <h1>{ current.title }</h1>
                    <h3>{ genres }</h3>
                    <p>{current.description}</p>
                </div>
                <div class="voter">
                    <div onclick = {ctx.link().callback(|_| Msg::Watch)} > {"Watch it !"}</div>
                    <div onclick = {ctx.link().callback(|_| Msg::Skip)} >{"Skip it..."}</div>
                </div>
            </div>
        </div>
        <div class="poster">
            <img src={current.poster_url}/>
        </div>
    </div>
</div>
        }
    }
}

