use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;

use crate::helpers::{Coordinate, Preference};

use super::AppState;
use crate::config::get_config;
use actix_web::web::Path;

#[derive(Deserialize)]
pub struct FspRequest {
    id: usize,
    waypoints: Vec<Coordinate>,
    alpha: Preference,
}

pub fn get_cost_tags() -> HttpResponse {
    HttpResponse::Ok().json(get_config().edge_cost_tags())
}

pub fn find_closest(query: web::Query<Coordinate>, state: web::Data<AppState>) -> HttpResponse {
    let graph = &state.graph;
    let coordinate = query.into_inner();

    let location = &graph.find_closest_node(&coordinate).location;
    HttpResponse::Ok().json(location)
}

pub fn fsp(
    req: HttpRequest,
    body: web::Json<FspRequest>,
    state: web::Data<AppState>,
) -> HttpResponse {
    match extract_token(&req) {
        None => HttpResponse::Unauthorized().finish(),
        Some(token) => {
            let mut users = state.users.lock().unwrap();
            let user_state = users.iter_mut().find(|x| x.auth.token == token);
            match user_state {
                None => HttpResponse::Unauthorized().finish(),
                Some(user) => {
                    let data = body.into_inner();
                    let id = data.id;
                    let path = state
                        .graph
                        .find_shortest_path_alt(id, data.waypoints, data.alpha);
                    if id != 0 {
                        user.update_route(path.as_ref());
                    }
                    HttpResponse::Ok().json(path)
                }
            }
        }
    }
}

pub fn get_preference(req: HttpRequest, state: web::Data<AppState>) -> HttpResponse {
    match extract_token(&req) {
        None => HttpResponse::Unauthorized().finish(),
        Some(token) => {
            let mut users = state.users.lock().unwrap();
            let user_state = users.iter_mut().find(|x| x.auth.token == token);
            match user_state {
                None => HttpResponse::Unauthorized().finish(),
                Some(user) => HttpResponse::Ok().json(&user.alphas),
            }
        }
    }
}

pub fn set_preference(
    req: HttpRequest,
    body: web::Json<Vec<Preference>>,
    state: web::Data<AppState>,
) -> HttpResponse {
    match extract_token(&req) {
        None => HttpResponse::Unauthorized().finish(),
        Some(token) => {
            let mut users = state.users.lock().unwrap();
            let user_state = users.iter_mut().find(|x| x.auth.token == token);
            match user_state {
                None => HttpResponse::Unauthorized().finish(),
                Some(user) => {
                    let new_alphas = body.into_inner();
                    user.alphas = new_alphas;
                    HttpResponse::Ok().finish()
                }
            }
        }
    }
}

pub fn new_preference(req: HttpRequest, state: web::Data<AppState>) -> HttpResponse {
    match extract_token(&req) {
        None => HttpResponse::Unauthorized().finish(),
        Some(token) => {
            let mut users = state.users.lock().unwrap();
            let user_state = users.iter_mut().find(|x| x.auth.token == token);
            match user_state {
                None => HttpResponse::Unauthorized().finish(),
                Some(user) => {
                    user.add_pref();
                    HttpResponse::Ok().json(&user.alphas)
                }
            }
        }
    }
}

pub fn find_preference(
    req: HttpRequest,
    state: web::Data<AppState>,
    body: web::Json<FspRequest>,
) -> HttpResponse {
    match extract_token(&req) {
        None => HttpResponse::Unauthorized().finish(),
        Some(token) => {
            let mut users = state.users.lock().unwrap();
            let user_state = users.iter_mut().find(|x| x.auth.token == token);
            match user_state {
                None => HttpResponse::Unauthorized().finish(),
                Some(user) => {
                    let body = body.into_inner();
                    let id = body.id;
                    let graph = &state.graph;
                    let mut route = graph
                        .find_shortest_path_alt(id, body.waypoints, body.alpha)
                        .unwrap();

                    graph.find_preference(&mut route);
                    if id == 0 {
                        user.add_route(&mut route);
                    } else {
                        user.update_route(Some(&route));
                    }
                    HttpResponse::Ok().json(&user.driven_routes)
                }
            }
        }
    }
}

pub fn get_routes(req: HttpRequest, state: web::Data<AppState>) -> HttpResponse {
    match extract_token(&req) {
        None => HttpResponse::Unauthorized().finish(),
        Some(token) => {
            let users = state.users.lock().unwrap();
            let user_state = users.iter().find(|x| x.auth.token == token);
            match user_state {
                None => HttpResponse::Unauthorized().finish(),
                Some(user) => {
                    let routes = &user.driven_routes;
                    HttpResponse::Ok().json(routes)
                }
            }
        }
    }
}

pub fn delete_route(
    req: HttpRequest,
    path: Path<usize>,
    state: web::Data<AppState>,
) -> HttpResponse {
    match extract_token(&req) {
        None => HttpResponse::Unauthorized().finish(),
        Some(token) => {
            let mut users = state.users.lock().unwrap();
            let user_state = users.iter_mut().find(|x| x.auth.token == token);
            match user_state {
                None => HttpResponse::Unauthorized().finish(),
                Some(user) => {
                    let id = path.into_inner();
                    user.delete_route(id);
                    HttpResponse::Ok().json(&user.driven_routes)
                }
            }
        }
    }
}

pub fn reset_data(req: HttpRequest, state: web::Data<AppState>) -> HttpResponse {
    match extract_token(&req) {
        None => HttpResponse::Unauthorized().finish(),
        Some(token) => {
            let mut users = state.users.lock().unwrap();
            let user_state = users.iter_mut().find(|x| x.auth.token == token);
            match user_state {
                None => HttpResponse::Unauthorized().finish(),
                Some(user) => {
                    user.reset();
                    HttpResponse::Ok().finish()
                }
            }
        }
    }
}

fn extract_token(req: &HttpRequest) -> Option<&str> {
    let auth_header = req.headers().get("Authorization");
    match auth_header {
        None => None,
        Some(value) => {
            let value = value.to_str().unwrap();
            if value.is_empty() {
                return None;
            }
            Some(value)
        }
    }
}
