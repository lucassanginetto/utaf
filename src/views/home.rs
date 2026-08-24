use templr::{templ, templ_ret};

use super::layout;

pub fn home_page() -> templ_ret![] {
    templ! {
        #layout("UTAF", true) {
            <main id="home_main">
                <h1>Welcome to UTAF</h1>
                <p>An alternative frontend for <a href="https://utaten.com">{"utaten.com"}</a></p>
                <form method="GET" action="/search">
                    <input id="home_search_input" type="text" name="layout_search_text" placeholder="Insert artist name or song title"/>
                    <div id="home_search_buttons_div">
                        <button class="home_search_button" type="submit" name="layout_search_type" value="title">Search by song title</button>
                        <button class="home_search_button" type="submit" name="layout_search_type" value="artist">Search by artist name</button>
                    </div>
                </form>
                <a href="/search">Advanced search</a>
            </main>
        }
    }
}
