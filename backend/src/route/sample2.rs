use crate::sample2_mod::{sample2_server::Sample2, Sample2Reply, Sample2Request};
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct MySample2 {}

#[tonic::async_trait]
impl Sample2 for MySample2 {
    async fn sample2(
        &self,
        _request: Request<Sample2Request>,
    ) -> Result<Response<Sample2Reply>, Status> {
        let reply = Sample2Reply {
            message: "Hello".to_string(),
        };

        Ok(Response::new(reply))
    }
}
