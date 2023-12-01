use lambda_http::{request::RequestContext, Request, RequestExt};

pub fn get_user_id(request: &Request) -> String {
    match request.request_context() {
        RequestContext::ApiGatewayV2(http) => http
            .authorizer
            .unwrap()
            .jwt
            .unwrap()
            .claims
            .get("sub")
            .unwrap()
            .to_owned(),
        RequestContext::ApiGatewayV1(_) => todo!(),
        RequestContext::Alb(_) => todo!(),
        RequestContext::WebSocket(_) => todo!(),
    }
}
