use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::pages::home::HomePage;

// Shell component remains the same
pub fn shell(options: LeptosOptions) -> impl IntoView {
    provide_meta_context();
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
                // Link to your CSS file
                 <Stylesheet id="leptos" href="/pkg/hours_counter.css"/>
                // Favicon links (example)
                // <link rel="shortcut icon" type="image/ico" href="/favicon.ico"/>
                // <link rel="icon" type="image/png" sizes="32x32" href="/favicon-32x32.png"/>
                // <link rel="icon" type="image/png" sizes="16x16" href="/favicon-16x16.png"/>

                 <Title text="Hours Counter"/> // Set the title
            </head>
            <body>
                 // Render the main App component here
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <MetaTags/>
        <Title text="Hours Counter"/>
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}
