// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused)]

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex, PoisonError};
use tauri::{generate_handler, State};

use tauri::Manager;

// Create an AuthState to manage whether we are authenticated or not
#[derive(Serialize, Clone)]
pub(crate) struct AuthState {
    // Ensure the token isn't passed to the frontend
    #[serde(skip_serializing)]
    token: Option<String>,
    // It's perfectly fine to just use a boolean to indicate logged_in
    logged_in: bool,
}
// Allow us to run AuthState::default()
impl Default for AuthState {
    fn default() -> Self {
        Self {
            // Before we log in we don't have a token
            token: None,
            // and we're not logged in
            logged_in: false,
        }
    }
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Io(std::io::Error),
    #[error("the mutex was poisoned")]
    PoisonError(String),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(err: std::sync::PoisonError<T>) -> Self {
        Error::PoisonError(err.to_string())
    }
}

// Create a command that logs us in
#[tauri::command()]
async fn login(state_mutex: State<'_, Mutex<AuthState>>) -> Result<AuthState, Error> {
    println!("Logging in");

    let mut state = state_mutex.lock()?;
    // Login logic
    state.logged_in = true;
    // Send back a clone of the State
    Ok(state.clone())
}

// Create a command that logs us out
#[tauri::command]
async fn logout(state_mutex: State<'_, Mutex<AuthState>>) -> Result<AuthState, String> {
    println!("Logging out");
    let mut state = state_mutex.lock().unwrap();
    // Logout logic
    state.logged_in = false;
    // Send back a clone of the State
    Ok(state.clone())
}

// Create a command that gets auth state
#[tauri::command]
async fn get_auth_state(state_mutex: State<'_, Mutex<AuthState>>) -> Result<AuthState, Error> {
    println!("Getting auth state");
    let state = state_mutex.lock()?;
    // Send back a clone of the State
    Ok(state.clone())
}

fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(AuthState::default()))
        .invoke_handler(tauri::generate_handler![login, logout, get_auth_state,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
