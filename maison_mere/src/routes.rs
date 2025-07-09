use rocket::{routes, Route};

use crate::controllers::{
    home_controller::*,
    rapport_controller::*,
    performance_controller::*
};

pub fn routes() -> Vec<Route> {
    routes![
        home,
        rapport,
        performances
    ]
}