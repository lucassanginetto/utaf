use templr::{templ, templ_ret};

pub fn head(title: &str) -> templ_ret!['_] {
    templ! {
        <head>
            <meta charset="UTF-8"/>
            <meta name="viewport" content="width=device-width, initial-scale=1"/>
            <title>{ title }</title>
            <link href="/static/style.css" rel="stylesheet" type="text/css"/>
            <link rel="icon" href="/static/favicon.svg" type="image/svg+xml"/>
            <script type="text/javascript" src="/static/script.js" defer/>
            <meta name="description" content="An alternative frontend for utaten.com"/>
        </head>
    }
}
