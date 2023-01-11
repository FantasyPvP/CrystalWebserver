use yew::prelude::*;
use std::{thread::sleep, time::Duration};
use std::path::PathBuf;




#[derive(Clone, PartialEq)]
struct Video {
    id: usize,
    title: String,
    description: String,
    url: String
}

#[derive(Properties, PartialEq)]
struct VideosLists {
    videos: Vec<Video>,
}

#[function_component(VideosList)]
fn videos_list(VideosLists { videos }: &VideosLists) -> Html {
    videos.iter().map(|video| html! {
        <div class="pic" key={video.id}>
            <h3>{format!("{}", video.title)}</h3>
            <iframe id="player" width="200" height="200" src={video.url.clone()}>
            </iframe>
            <p>{format!("{}", video.description)}</p>
        </div>
    }).collect::<Html>()
}



#[derive(Clone, PartialEq)]
struct Pack {
    name: String,
    desc: String,
    author: String,
    idx: usize,
    path: String,
    icon: String,
}

#[derive(Properties, PartialEq)]
struct PackList {
    packs: Vec<Pack>
}



#[function_component(PackLister)]
fn pack_list(PackList { packs }: &PackList) -> Html {
    packs.iter().map(|pack| html!{
        <div class="flex bg-zinc-900 rounded-2xl shadow-2xl shadow-black hover:border-indigo-800 hover:bg-zinc-800 hover:border-8 ">
            <img class="rounded-xl border-indigo-800 object-contain w-2/5" src={format!("assets/pack/ico/{}", pack.icon)} alt="logo"/>
            <div class="py-5 w-3/5">
                <p class="p-6 text-center text-slate-400 text-2xl hover:text-indigo-800">{&pack.name}</p>
                <p class="px-4 text-center text-slate-500 text-m">{&pack.desc}</p>
            </div>
        </div>
    }).collect::<Html>()
}















#[function_component(App)]
fn app() -> Html {

    let packs = vec![
        Pack {
            name: "256".to_string(),
            desc: "the best pack".to_string(),
            author: "fantasypvp".to_string(),
            idx: 1,
            path: "/home/packs/idk".to_string(),
            icon: "pack.png".to_string(),
        },
        Pack {
            name: "512".to_string(),
            desc: "the best pack v512".to_string(),
            author: "fantasypvp".to_string(),
            idx: 1,
            path: "/home/packs/idk".to_string(),
            icon: "pack2.png".to_string(),
        },
        Pack {
            name: "512 pro".to_string(),
            desc: "the pro pack for absolute legends, using this pack makes you a god at the game fr fr so dont hesitate to buy it for the cheap price of £29.99".to_string(),
            author: "fantasypvp".to_string(),
            idx: 1,
            path: "/home/packs/idk".to_string(),
            icon: "pack3.png".to_string(),
        }
    ];








    let text = String::from("
Welcome to the Only Bozos and Prodigies website! The purpose of this website is for memes and apparently make money
but its mostly just an excuse for me to learn html and css so i dont look like an idiot compared to the average js
developer which would not be fun lmfao");


    let text2 = String::from("
    cats are superior
    they really just are
    you better start coping
    or i'll shunt you, AHA
    ");


    html! {
        <>
            <body class="h-fill bg-zinc-900">
                <div class="p-5 min-w-fit mx-auto bg-slate-800 rounded-b-lg shadow-lg flex flex-row items-center space-x-12">
                    <div class="shrink-0">
                        <img class="h-32 w-32 rounded-lg border-4 border-slate-700 shadow-lg hover:bg-slate-700 hover:border-indigo-800 hover:h-48 hover:w-48" src="assets/logo.svg" alt="logo"/>
                    </div>

                    <div class="flex flex-col">
                        <h2 class="text-3xl py-2 font-medium text-white">{"Only Bozos and Prodigies"}</h2>
                        <p class="px-5 whitespace-pre-wrap text-transparent bg-clip-text bg-gradient-to-r from-indigo-400 to-pink-600">{">   we make cool stuff"}</p>
                        <p class="px-5 whitespace-pre-wrap text-transparent bg-clip-text bg-gradient-to-r from-indigo-400 to-pink-600">{">   _"}</p>
                        //<textarea oninput=>
                    </div>
                </div>

                <div class="min-h-fit min-w-fit p-10 space-y-12">
                    <div class="rounded-lg border-4 border-slate-700 min-w-fit bg-slate-800 shadow-lg hover:bg-slate-700 hover:border-indigo-800">
                        <div class="p-10">
                            <p class="text-slate-400 hover:font-black hover:text-xl hover:text-transparent hover:bg-clip-text hover:bg-gradient-to-r hover:from-indigo-400 hover:to-pink-600">
                            {
                                text
                            }</p>
                            <p class="text-slate-400 hover:font-black hover:text-xl hover:text-transparent hover:bg-clip-text hover:bg-gradient-to-r hover:from-indigo-400 hover:to-pink-600">
                            {
                                text2
                            }</p>

                        </div>
                    </div>

                    <div class="bg-slate-800 shadow-lg hover:bg-slate-700 hover:border-indigo-800 rounded-xl border-4 border-slate-700">
                        <div class="w-fill h-20">
                            <h2 class="text-4xl py-10 font-medium text-slate-400 text-center
                            hover:font-black hover:text-transparent hover:bg-clip-text hover:bg-gradient-to-r hover:from-indigo-400 hover:to-pink-600">
                            {"My PvP packs!"}
                            </h2>
                        </div>
                        <div class="grid grid-cols-1 2xl:grid-cols-2 3xl:grid-cols-3 p-10 gap-10 gap-y-10">
                            <PackLister packs={packs}/>
                        </div>
                    </div>

                    <div class="grid grid-cols-3 rounded-xl border-4 border-slate-700 p-10 gap-10 gap-y-10
                    bg-slate-800 shadow-lg hover:bg-slate-700 hover:border-indigo-800">
                        <img class="rounded-2xl shadow-2xl shadow-black hover:border-8 hover:border-indigo-800 object-cover" src="assets/imgs/cat1.png" alt="idefk"/>
                        <img class="rounded-2xl shadow-2xl shadow-black hover:border-8 hover:border-indigo-800 object-cover" src="assets/imgs/cat2.png" alt="idefk"/>
                        <img class="rounded-2xl shadow-2xl shadow-black hover:border-8 hover:border-indigo-800 object-cover" src="assets/imgs/cat3.png" alt="idefk"/>
                        <img class="rounded-2xl shadow-2xl shadow-black hover:border-8 hover:border-indigo-800 object-cover" src="assets/imgs/cat4.png" alt="idefk"/>
                        <img class="rounded-2xl shadow-2xl shadow-black hover:border-8 hover:border-indigo-800 object-cover" src="assets/imgs/cat5.png" alt="idefk"/>
                        <img class="rounded-2xl shadow-2xl shadow-black hover:border-8 hover:border-indigo-800 object-cover" src="assets/imgs/cat6.png" alt="idefk"/>
                        <img class="rounded-2xl shadow-2xl shadow-black hover:border-8 hover:border-indigo-800 object-cover" src="assets/imgs/cat7.png" alt="idefk"/>

                        </div>


                </div>
            </body>
        </>
    }
}







//    let counter = use_state(|| 0);
//    let timer = use_state(|| 30);

//    let onclick = {
//        let counter = counter.clone();
//        move |_| {
//            let value = *counter + 1;
//            counter.set(value);
//        }
//    };

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




    /*            <div>
                    <VideosList videos={videos}/>
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
                </div> */

    /*
    let videos = vec![
        Video {
            id: 1,
            title: "never gonna give you up".to_string(),
            description: "by rick arsely".to_string(),
            url: "https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string()
        },
        Video {
            id: 1,
            title: "never gonna give you up".to_string(),
            description: "by rick arsely".to_string(),
            url: "https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string()
        }
    ]; */




fn main() {

    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}
