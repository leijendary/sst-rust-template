use lambda_http::{request::RequestContext, Request, RequestExt};
use model::error::{unauthorized, ErrorResult};

pub fn get_user_id(request: &Request) -> Result<String, ErrorResult> {
    match request.request_context() {
        RequestContext::ApiGatewayV2(http) => http
            .authorizer
            .and_then(|a| a.jwt)
            .map(|j| j.claims)
            .and_then(|c| c.get("sub").map(|s| s.to_owned()))
            .ok_or(unauthorized()),
        RequestContext::ApiGatewayV1(_) => todo!(),
        RequestContext::Alb(_) => todo!(),
        RequestContext::WebSocket(_) => todo!(),
    }
}
