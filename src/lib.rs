//! # Replicate Rust Client
//!
//! This is a Rust client for Replicate's API.
//!
//! See https://replicate.com/docs for more information.

// Declare modules
mod account;
mod api;
mod backoff;
mod client;
mod collection;
mod deployment;
mod error;
mod examples;
mod files;
mod identifier;
mod model;
mod paginate;
mod prediction;
mod run;
mod status;
mod stream;
mod training;
mod wait;
mod webhook;
mod identifier_test;
// Re-export main structs and functions
pub use crate::account::Account;
pub use crate::backoff::{ Backoff, ExponentialBackoff };
pub use crate::client::Client;
pub use crate::collection::Collection;
pub use crate::deployment::{ Deployment, CreateDeploymentOptions, UpdateDeploymentOptions };
pub use crate::error::ModelError;
pub use crate::files::{ File, CreateFileOptions };
pub use crate::identifier::{ Identifier, InvalidIdentifierError };
pub use crate::model::{ Model, ModelVersion, CreateModelOptions };
pub use crate::paginate::Page;
pub use crate::prediction::{
    Prediction,
    PredictionInput,
    PredictionOutput,
    PredictionProgress,
    PredictionMetrics,
    Source,
};
pub use crate::status::Status;
pub use crate::stream::{ SSEEvent, InvalidUTF8DataError };
pub use crate::training::Training;
pub use crate::webhook::{ Webhook, WebhookEvent };
