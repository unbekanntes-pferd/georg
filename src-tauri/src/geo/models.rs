use serde::Serialize;

use crate::parse::ChildCareRequestResponse;

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
