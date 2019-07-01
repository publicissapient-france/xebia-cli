use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    Scope, TokenResponse, TokenUrl,
};
use serde::ser::Serialize;
use std::io::{self, Read};
use url::Url;

pub fn auth() {
    // Create an OAuth2 client by specifying the client ID, client secret, authorization URL and
    // token URL.
    let client = BasicClient::new(
        ClientId::new("Af3AamedzFSCHmGZhdZZRZLD4VV4D6j4".to_string()),
        Some(ClientSecret::new("client_secret".to_string())),
        AuthUrl::new(
            Url::parse("https://xdd.eu.auth0.com/authorize").expect("Failed to parse AuthURL"),
        ),
        Some(TokenUrl::new(
            Url::parse("https://xdd.eu.auth0.com/oauth/token").expect("Fail o parse TokenURL"),
        )),
    )
    // Set the URL the user will be redirected to after the authorization process.
    .set_redirect_url(RedirectUrl::new(
        Url::parse("http://localhost:8888").expect("Failed to parse RedirectURL"),
    ));

    // Generate a PKCE challenge.
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    //println!("Verifier: {:?}", pkce_verifier.);

    // Generate the full authorization URL.
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        .add_scope(Scope::new("offline_access".to_string()))
        .add_extra_param("audience", "https://api.xebia.fr")
        // Set the PKCE code challenge.
        .set_pkce_challenge(pkce_challenge)
        .url();
    // TODO state
    // TODO scope
    // TODO audience cfg

    // This is the URL you should redirect the user to, in order to trigger the authorization
    // process.
    println!(
        "Please browse to this URL to grant access to Xebia CLI on XDD:\n    =>  {}",
        auth_url
    );
    let mut buffer = String::new();
    println!("What's your token (the 'code' part of the URL you will be redirected to) ?");
    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Failed to read token on stdin");

    // Once the user has been redirected to the redirect URL, you'll have access to the
    // authorization code. For security reasons, your code should verify that the `state`
    // parameter returned by the server matches `csrf_state`.

    println!("{:?}", buffer.trim().to_string());
    // Now you can trade it for an access token.
    let token_result = client
        .exchange_code(AuthorizationCode::new(buffer.trim().to_string()))
        // Set the PKCE code verifier.
        .set_pkce_verifier(pkce_verifier)
        .add_extra_param("client_id", "Af3AamedzFSCHmGZhdZZRZLD4VV4D6j4")
        .request(http_client)
        .expect("Failed to get token result");
    log::info!("Successfully retrieved Auth0 token!");
    println!("Run this to export the secret to your env in order to be able to use this CLI:\nexport XDD_API_KEY={:?}", token_result.access_token().secret());

    // Unwrapping token_result will either produce a Token or a RequestTokenError.
}
