use crate::std_format_msgs::OutgoingWsMsg;
use actix::prelude::*;
use engine_actor::Engine;
use game_actor::Game;
use rand::{self, rngs::ThreadRng, Rng};
use serde::Serialize;
use server::server_actor::Server;
use sockets::SocketSession;
use sockets::{socket_endpoint, socket_api::SocketAPI, socket_api::SocketSessionAPI};
use std::collections::HashMap;
use std::time::Duration;
use game_actor::TimeFormat;
use serde_json::Error as JsonErr;
use socket_endpoint::SessionContext;
use std::time::Instant;
use crate::std_format_msgs::IncomingWsMsg;
use actix_web_actors::ws::{self, WebsocketContext};
use server::GameAPI;
use serde_json::Error as JsonError;
use server::UserAPI;
use crate::std_format_msgs::content_templates as content;
use crate::std_format_msgs::*;

// Files

pub mod engine_actor;
pub mod game_actor;
pub mod server;
pub mod sockets;

pub mod auth;
pub mod types;
pub mod validate;