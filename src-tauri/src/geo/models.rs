use serde::Serialize;

use crate::parse::{AssistantResponse, CandidateResponse, ChildCareRequestResponse};

#[derive(Debug, Clone, Serialize)]
pub struct ChildCareRequestMatch {
    pub candidate: ChildCareRequestResponse,
    pub distance: f64,
}

impl ChildCareRequestMatch {
    pub fn new(candidate: ChildCareRequestResponse, distance: f64) -> Self {
        Self {
            candidate,
            distance,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CandidateMatch {
    pub candidate: CandidateResponse,
    pub distance: f64,
}

impl CandidateMatch {
    pub fn new(candidate: CandidateResponse, distance: f64) -> Self {
        Self {
            candidate,
            distance,
        }
    }
}


#[derive(Debug, Clone, Serialize)]
pub struct AssistantMatch {
    pub assistant: AssistantResponse,
    pub distance: f64,
}

impl AssistantMatch {
    pub fn new(assistant: AssistantResponse, distance: f64) -> Self {
        Self {
            assistant,
            distance,
        }
    }
}