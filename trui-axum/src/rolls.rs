use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::darkmode::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Roll {
    id: u16,
    sbls: String,
    completed: bool,
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        // use sqlx::{Connection, SqliteConnection};
        // use http::{header::SET_COOKIE, HeaderMap, HeaderValue, StatusCode};

        // pub async fn db() -> Result<SqliteConnection, ServerFnError> {
        //     let mut conn = SqliteConnection::connect("sqlite:./trui-axum/rolls.db?mode=rwc").await?;
        //     let create_table_query = r#"
        //         CREATE TABLE IF NOT EXISTS rolls (
        //             id INTEGER PRIMARY KEY,
        //             title TEXT NOT NULL,
        //             completed INTEGER NOT NULL
        //         )
        //     "#;
        //
        //     // Execute the SQL statement to create the table
        //     sqlx::query(&create_table_query).execute(&mut conn).await?;
        //     
        //     Ok(conn)
        // }
    }
}

#[component]
pub fn RollsApp() -> impl IntoView {
    //let id = use_context::<String>();
    provide_meta_context();
    view! {
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Stylesheet id="leptos" href="/pkg/todo_app_sqlite_axum.css"/>
        <Router>
            <header>
                <h1>"My Roll App"</h1>
            </header>
            <main>
                <DarkModeToggle/>
                <Routes>
                    <Route path="" view=RollSearch/>
                </Routes>
            </main>
        </Router>
    }
}

#[server(SearchRolls)]
pub async fn search_rolls(sbls: String) -> Result<(), ServerFnError> {
    // fake API delay
    let sbls = sbls.split(",").map(|sbl| sbl.trim()).collect::<Vec<_>>();
    logging::log!("sbls: {sbls:#?}");

    Ok(())
}

#[component]
pub fn RollSearch() -> impl IntoView {
    let search_rolls = create_server_multi_action::<SearchRolls>();

    view! {
        <div>
            <MultiActionForm action=search_rolls>
                <label>
                    "Search SBLs: "
                    <input type="text" name="sbls"/>
                </label>
                <input type="submit" value="Search"/>
            </MultiActionForm>
        </div>
    }
}
