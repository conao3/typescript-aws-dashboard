use actix_web::{HttpMessage, HttpRequest, dev::ServiceRequest};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct TenantContext {
    pub tenant_id: Uuid,
    pub user_id: Uuid,
    pub email: String,
}

pub fn extract_tenant_context(
    req: &HttpRequest,
    jwt_config: &crate::auth::JwtConfig,
) -> Option<TenantContext> {
    let auth_header = req.headers().get("authorization")?;
    let auth_str = auth_header.to_str().ok()?;

    if !auth_str.starts_with("Bearer ") {
        return None;
    }

    let token = &auth_str[7..];
    let claims = jwt_config.verify_token(token).ok()?;

    Some(TenantContext {
        tenant_id: claims.tenant_id,
        user_id: claims.sub,
        email: claims.email,
    })
}

pub fn extract_tenant_context_from_service_request(
    req: &ServiceRequest,
    jwt_config: &crate::auth::JwtConfig,
) -> Option<TenantContext> {
    let auth_header = req.headers().get("authorization")?;
    let auth_str = auth_header.to_str().ok()?;

    if !auth_str.starts_with("Bearer ") {
        return None;
    }

    let token = &auth_str[7..];
    let claims = jwt_config.verify_token(token).ok()?;

    Some(TenantContext {
        tenant_id: claims.tenant_id,
        user_id: claims.sub,
        email: claims.email,
    })
}
