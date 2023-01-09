use yew::prelude::*;
use std::{thread::sleep, time::Duration};


struct Video {
    id: usize,
    title: String,
    description: String,
    url: String
}



#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let timer = use_state(|| 30);

    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    /*let init = {
        let timer = timer.clone();
        move |_| {
            //for i in (0..3).rev() {
                //sleep(Duration::from_secs(1));
                let timervalue = *timer - 1;
                timer.set(timervalue);
            //};
        }
    };*/

    let videos = vec![
        Video {
            id: 1,
            title: "never gonna give you up".to_string(),
            description: "by rick arsely".to_string(),
            url: "https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string()
        }
    ];

    let videos = videos.iter().map(|video| html! {
        <div class='pic'>
            <h3>{format!("{}", video.title)}</h3>
            <iframe id="player" width="200" height="200" src={video.url.clone()}>
            </iframe>
            <p>{format!("{}", video.description)}</p>
        </div>
    }).collect::<Html>();











    html! {
        <>
            <h1>{"The cat army is coming for you"}</h1>
            <div>
            {videos}
                <button {onclick}>{ "+1" }</button>
                <p>{ *counter }</p>

                <p1>{"the ultimate weapon is approaching, your options:"}</p1>
                <ul>
                    <li>{"run."}</li>
                    <li>{"hide."}</li>
                    <li>{"meet your demise."}</li>
                </ul>
            </div>
            <div>
                <h3>{"fear it."}</h3>
                <img src ="https://media.discordapp.net/attachments/891575462118170654/1061980312831266816/cat_swordsman.png" alt="idefk"/>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
