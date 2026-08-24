use templr::{templ, templ_ret};

use super::{footer, head, navbar};

pub fn layout<'a>(title: &'a str, hide_nav_search: bool) -> templ_ret!['a] {
    templ! {
        #use children;
        <!DOCTYPE html>
        <html lang="en">
            #head(title);
            <body>
                #navbar(hide_nav_search);
                #children;
                #footer();
            </body>
        </html>
    }
}
