use crate::account_manager;
use crate::apis::ApiError;
use crate::auth_verifier::AdminToken;
use account_manager::AccountManager;
use rocket::serde::json::Json;
use rsky_lexicon::com::atproto::server::{
    AccountCodes, CreateInviteCodeInput, CreateInviteCodeOutput,
};
use crate::db::DbConn;

#[tracing::instrument(skip_all)]
#[rocket::post(
    "/xrpc/com.atproto.server.createInviteCode",
    format = "json",
    data = "<body>"
)]
pub async fn create_invite_code(
    body: Json<CreateInviteCodeInput>,
    db: DbConn,
    _auth: AdminToken,
) -> Result<Json<CreateInviteCodeOutput>, ApiError> {
    // @TODO: verify admin auth token
    let CreateInviteCodeInput {
        use_count,
        for_account,
    } = body.into_inner();
    let code = super::gen_invite_code();

    match AccountManager::create_invite_codes(
        vec![AccountCodes {
            codes: vec![code.clone()],
            account: for_account.unwrap_or("admin".to_owned()),
        }],
        use_count,
        &db,
    )
    .await
    {
        Ok(_) => Ok(Json(CreateInviteCodeOutput { code })),
        Err(error) => {
            tracing::error!("Internal Error: {error}");
            Err(ApiError::RuntimeError)
        }
    }
}
