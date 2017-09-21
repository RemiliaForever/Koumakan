mod article;
mod comment;
mod label_archive;

use rocket::Route;
pub use self::label_archive::ALCache;


pub fn get_routes() -> Vec<Route> {
    routes![
        label_archive::get_archive,
        label_archive::get_label,
        article::get_article_list,
        article::get_article,
        article::get_article_nav,
        article::add_article,
        comment::get_comments,
        comment::add_comment,
    ]
}
