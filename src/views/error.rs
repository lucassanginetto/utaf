use reqwest::StatusCode;
use templr::{templ, templ_ret};

use super::layout;

pub fn error_page(code: StatusCode, display: &str) -> templ_ret!['_] {
    templ! {
        #layout("Error - UTAF", false) {
            <main id="error_main">
                <h1>{ code.as_str() }</h1>
                <p>{ display }</p>
            </main>
        }
    }
}
