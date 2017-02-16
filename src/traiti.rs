
use std::sync::Arc;

enum Er {}
type Result<T> = std::result::Result<T, Er>;
struct AppendEntriesRequest{}
struct AppendEntriesResponse{}

trait InjectRun {
    fn invoke(&self, i: AppendEntriesRequest) -> ::Result<AppendEntriesResponse>;
}

trait RaftRpc {}
trait Rde: RaftRpc {}

struct RdeS{}

impl RaftRpc for RdeS{}
impl<T> InjectRun for T
    where T : RaftRpc
{
    // REQUIRE SIZED!
    fn invoke(&self, i: AppendEntriesRequest) -> ::Result<AppendEntriesResponse> {
        Ok(AppendEntriesResponse{})
    }
}

impl InjectRun for RaftRpc
{
    fn invoke(&self, i: AppendEntriesRequest) -> ::Result<AppendEntriesResponse> {
        Ok(AppendEntriesResponse{})
    }
}

struct HyperRpcServerHandler {
    // handler: Arc<Box<RaftRpc>>,
    handler: RdeS,
}

impl HyperRpcServerHandler {
    fn go(&self){
        let c = &self.handler;//.as_ref();
        (c).invoke(AppendEntriesRequest {});
    }
}

fn main(){
}