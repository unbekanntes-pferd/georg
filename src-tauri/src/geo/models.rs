use serde::Serialize;

use crate::parse::ChildCareRequest;

#[derive(Debug, Clone, Serialize)]
pub struct ChildCareRequestMatch {
    pub candidate: ChildCareRequest,
    pub distance: f64,
}

impl ChildCareRequestMatch {
    pub fn new(candidate: ChildCareRequest, distance: f64) -> Self {
        Self {
            candidate,
            distance,
        }
    }
}