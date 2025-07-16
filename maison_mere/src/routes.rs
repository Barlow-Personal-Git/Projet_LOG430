use rocket::{routes, Route};

use crate::controllers::{home_controller::*, performance_controller::*, rapport_controller::*};

pub fn routes() -> Vec<Route> {
    routes![home, rapport, performances]
}
