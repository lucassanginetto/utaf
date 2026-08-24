use templr::{templ, templ_ret};

pub fn footer() -> templ_ret![] {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    templ! {
        <footer>
            <a target="_blank" href="https://github.com/lucassanginetto/utaf">Source Code</a>
            <p>{ format!("v{VERSION}") }</p>
        </footer>
    }
}
