use templr::{templ, templ_ret};

pub fn navbar(hide_search: bool) -> templ_ret![] {
    templ! {
        <nav>
            <a href="/"><img src="/static/logo.png" alt="UTAF" width="100px" height="25px"/></a>
            #if !hide_search {
                <form method="GET" action="/search">
                    <input type="text" name="layout_search_text" id="navbar_search_input" placeholder="Search..."/>
                    <input type="hidden" name="layout_search_type" value="title"/>
                </form>
            }
            <a
                title="Go to utaten.com"
                alt="Go to utaten.com"
                id="goto_utaten_a"
                rel="noopener noreferrer"
                target="_blank"
            >
                <svg
                    width="25px"
                    height="25px"
                    viewbox="0 0 24 24"
                    fill="none"
                    xmlns="http://www.w3.org/2000/svg"
                >
                    <path
                        stroke="black"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        d="M10 4H6C4.89543 4 4 4.89543 4 6V18C4 19.1046 4.89543 20 6 20H18C19.1046 20 20 19.1046 20 18V14M11 13L20 4M20 4V9M20 4H15"
                    />
                </svg>
            </a>
        </nav>
    }
}
