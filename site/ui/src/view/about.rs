use crate::utils::navigation_utils;
use yew::prelude::*;

#[function_component(About)]
pub fn about_function_component() -> Html {
    use_effect_with_deps(
        |_| {
            navigation_utils::scroll_to_top();
        },
        (),
    );

    return html! {
        <section>
            <div class="container mt-1 p-4 bg-dark">
                 <div class="row justify-content-md-center text-center mb-4">
                    <p class="text-white fs-2">{"IlluviAnalytics is a passion project of"}</p>
                    <div class="col-md-4 animate__animated animate__fadeInLeft animate__fast">
                      <img src="/img/enigo.jpg" class="img-fluid border border-2 border-light rounded-4 shadow-gradient" width="250" height="250" alt="Enigo"/>
                      <p class="text-white fs-4">{"Enigo"}</p>
                    </div>
                    <div class="col-md-4 animate__animated animate__fadeInRight animate__fast">
                      <img src="/img/angel.jpg" class="img-fluid border border-2 border-light rounded-4 shadow-gradient" width="250" height="250" alt="Angel"/>
                      <p class="text-white fs-4">{"Angel"}</p>
                    </div>
                 </div>
                 <div class="row justify-content-md-center text-center mb-4 g-1">
                    <p class="text-white fs-4">{"Following API providers are used to fetch the data"}</p>
                    <div class="col-md-5 border rounded">
                        <ul class="list-group">
                          <li class="list-group-item bg-dark"><a href="https://docs.x.immutable.com" target="_blank" class="text-decoration-none fs-5">{"ImmutableX"}</a></li>
                          <li class="list-group-item bg-dark"><a href="https://www.coingecko.com" target="_blank" class="text-decoration-none fs-5">{"CoinGecko"}</a></li>
                          <li class="list-group-item bg-dark"><a href="https://docs.etherscan.io" target="_blank" class="text-decoration-none fs-5">{"Etherscan"}</a></li>
                        </ul>
                    </div>
                    <p class="text-white fs-6 mt-3">{"All prices on the site are displayed in buyer (aka taker) denomination
                                which includes royalty, ecosystem and protocol fees (where applicable)"}</p>
                 </div>
            </div>
            <div class="container-fluid p-4 bg-gray">
                <div class="container">
                     <div class="row justify-content-md-center text-center mb-4">
                        <p class="text-white fs-2">{"Roadmap"}</p>
                        <p class="text-white fs-4 mb-0">{"This is just the beginning!"}</p>
                        <p class="text-white fs-4 mb-0">{"While our focus for this initial version of the website was on utility, we have an exciting roadmap ahead filled with planned improvements and additional features."}</p>
                        <p class="text-white fs-4 mb-0">{"Get ready for an enhanced user experience and even more valuable insights."}</p>
                        <p class="text-white fs-4">{"Here's a glimpse of what's to come:"}</p>
                        <div class="col-md-5 p-0 border rounded">
                            <ul class="list-group">
                              <li class="list-group-item bg-dark"><p class="text-white fs-5 m-1">{"Adding new page for Illuvium Universe monetary data"}</p></li>
                              <li class="list-group-item bg-dark"><p class="text-white fs-5 m-1">{"Adding new page for wallet stats"}</p></li>
                              <li class="list-group-item bg-dark"><p class="text-white fs-5 m-1">{"Assets Explore UI overhaul"}</p></li>
                              <li class="list-group-item bg-dark"><p class="text-white fs-5 m-1">{"General UI overhaul (because we can make it better)"}</p></li>
                            </ul>
                        </div>
                        <p class="text-white fs-5 mt-4 mb-1">{"Have feedback to share? That's awesome, send it over!"}</p>
                        <a href="mailto:info@pudding.pro" class="text-decoration-none fs-5">{"info@pudding.pro"}</a>
                     </div>
                 </div>
            </div>
            <div class="container p-4 bg-dark">
                <div class="row justify-content-md-center text-center mt-4">
                    <p class="text-muted mb-0">
                        <small>{"Logo and portrait images generated by "}
                            <a href="https://www.bing.com/images/create" target="_blank" class="text-decoration-none">{"Bing Image Creator"}</a>
                        </small>
                    </p>
                </div>
            </div>
        </section>
    };
}
