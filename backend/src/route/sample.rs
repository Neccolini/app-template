use crate::sample_mod::{sample_server::Sample, SampleReply, SampleRequest};
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct MySample {}

#[tonic::async_trait]
impl Sample for MySample {
    async fn sample(
        &self,
        _request: Request<SampleRequest>,
    ) -> Result<Response<SampleReply>, Status> {
        let reply = SampleReply {
            message: "Hello".to_string(),
        };

        Ok(Response::new(reply))
    }
}
