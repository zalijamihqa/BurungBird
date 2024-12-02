use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct App {
    link: ComponentLink<Self>,
    threads: Vec<String>,
    ads: Vec<String>,
    events: Vec<String>,
}

enum Msg {
    FetchThreads,
    FetchAds,
    FetchEvents,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            link: ctx.link().clone(),
            threads: vec![],
            ads: vec![],
            events: vec![],
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchThreads => {
                // Simulasi pengambilan data thread
                self.threads = vec![
                    "Thread: Cara memelihara burung kenari".into(),
                    "Thread: Makanan terbaik untuk burung lovebird".into(),
                ];
                true
            }
            Msg::FetchAds => {
                // Simulasi pengambilan data ads
                self.ads = vec![
                    "Ad: Dijual burung kakatua - Rp 2.500.000".into(),
                    "Ad: Dijual kandang burung ukuran besar - Rp 500.000".into(),
                ];
                true
            }
            Msg::FetchEvents => {
                // Simulasi pengambilan data event
                self.events = vec![
                    "Event: Lomba kicau burung di Jakarta - 10 Desember".into(),
                    "Event: Kontes burung hias di Surabaya - 15 Desember".into(),
                ];
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{ "BurungBird" }</h1>
                <div>
                    <h2>{ "Forum Diskusi" }</h2>
                    <button onclick={ctx.link().callback(|_| Msg::FetchThreads)}>{ "Lihat Thread" }</button>
                    <ul>
                        { for self.threads.iter().map(|thread| html! { <li>{ thread }</li> }) }
                    </ul>
                </div>
                <div>
                    <h2>{ "Marketplace" }</h2>
                    <button onclick={ctx.link().callback(|_| Msg::FetchAds)}>{ "Lihat Iklan" }</button>
                    <ul>
                        { for self.ads.iter().map(|ad| html! { <li>{ ad }</li> }) }
                    </ul>
                </div>
                <div>
                    <h2>{ "Event Finder" }</h2>
                    <button onclick={ctx.link().callback(|_| Msg::FetchEvents)}>{ "Lihat Event" }</button>
                    <ul>
                        { for self.events.iter().map(|event| html! { <li>{ event }</li> }) }
                    </ul>
                </div>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<App>::new().mount_to_body();
}
