use yew::prelude::*;

use crate::model::Movie;

#[derive(Properties, PartialEq)]
pub  struct  VoterProperties {
    pub movie: Movie,
    pub on_watch: Callback<String>,
    pub on_skip: Callback<String>,
}

#[function_component(Voter)]
pub fn voter(
    VoterProperties { movie, on_watch, on_skip }: &VoterProperties,
) -> Html {

    let movie = movie.clone();

    let movie_id = movie.id.clone();
    let on_watch = {
        let on_watch = on_watch.clone();
        Callback::from(move |_| {
            on_watch.emit(movie_id.clone())
        })
    };

    let movie_id = movie.id.clone();
    let on_skip = {
        let on_skip = on_skip.clone();
        Callback::from(move |_| {
            on_skip.emit(movie_id.clone())
        })
    };

    html! {
            <div class="content">
                <div class="text">
                    <h1>{ movie.title }</h1>
                    <h3>{ movie.genres.join(" | ") }</h3>
                    <p>{ movie.description }</p>
                </div>
                <div class="voter">
                    <div onclick = { on_watch } > {"Watch it !"} </div>
                    <div onclick = { on_skip } > {"Skip it..."} </div>
                </div>
            </div>
    }
}