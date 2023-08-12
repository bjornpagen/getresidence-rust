use std::{fmt::Display, str::FromStr};

use base64;
use branca::Branca;
use email_address::EmailAddress;
use http::StatusCode;
use maud::{html, PreEscaped, DOCTYPE};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use worker::*;

mod dubai;
mod styles;

macro_rules! ensure {
    ($expr:expr, $msg:expr) => {
        if !$expr {
            return Err($msg.into());
        }
    };
}

fn default_headers(nonce: Option<&str>) -> Headers {
    let mut headers = Headers::new();
    let mut csp = "default-src 'none'; img-src data: imagedelivery.net; font-src use.typekit.net; connect-src 'self'".to_owned();
    if let Some(nonce) = nonce {
        csp.push_str(&format!(
            "; script-src 'nonce-{0}' 'strict-dynamic'; style-src 'nonce-{0}'",
            nonce
        ));
    }
    headers
        .set("Content-Type", "text/html; charset=utf-8")
        .unwrap();
    headers.set("Content-Security-Policy", &csp).unwrap();
    headers.set("Referrer-Policy", "no-referrer").unwrap();
    headers.set("Permissions-Policy", "accelerometer=(), ambient-light-sensor=(), autoplay=(), battery=(), camera=(), display-capture=(), document-domain=(), encrypted-media=(), execution-while-not-rendered=(), execution-while-out-of-viewport=(), fullscreen=(), gamepad=(), geolocation=(), gyroscope=(), hid=(), identity-credentials-get=(), idle-detection=(), local-fonts=(), magnetometer=(), microphone=(), midi=(), otp-credentials=(), payment=(), picture-in-picture=(), publickey-credentials-create=(), publickey-credentials-get=(), screen-wake-lock=(), serial=(), speaker-selection=(), storage-access=(), usb=(), web-share=(), xr-spatial-tracking=()").unwrap();
    headers.set("X-Frame-Options", "DENY").unwrap();
    headers
}

fn emoji_to_favicon(emoji: &str) -> String {
    format!("data:image/svg+xml,<svg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%220 0 100 100%22><text y=%22.9em%22 font-size=%2290%22>{}</text></svg>", emoji)
}

fn get_field(data: FormData, field: &str) -> Result<String> {
    let name_entry = data
        .get(field)
        .ok_or(format!("form field does not exist: {}", field))?;

    match name_entry {
        FormEntry::Field(n) => Ok(n),
        _ => Err("expected a form field".into()),
    }
}

// base_layout is the base HTML template we will use for all responses.
pub fn base_layout(
    domain: &'static str,
    icon: &'static str,
    title: &'static str,
    description: &'static str,
    main: PreEscaped<String>,
    nonce: &str,
) -> PreEscaped<String> {
    html! {
        (DOCTYPE)
        meta charset="utf-8" {}
        html lang="en" {
            head {
                title { (PreEscaped(title)) }
                meta name="description" content=(PreEscaped(description)) {}
                link rel="icon" href=(PreEscaped(emoji_to_favicon(icon))) {}
                meta property="og:locale" content="en_US" {}
                meta property="og:title" content=(PreEscaped(title)) {}
                meta property="og:description" content=(PreEscaped(description)) {}
                // TODO: meta property="og:image" content="https://example.com/image.jpg" {}
                // TODO: meta property="og:image:alt" content="A description of what is in the image (not a caption)." {}
                meta name="viewport" content="width=device-width, initial-scale=1" {}
                style #reset nonce=(nonce) {
                    (PreEscaped(styles::RESET))
                }
                style #global nonce=(nonce) {
                    (PreEscaped(styles::SYSTEM))
                }
                link #fonts rel="preload" href="https://use.typekit.net/gnn8txw.css" as="style" nonce=(nonce) {}
                script nonce=(nonce) {
                    (PreEscaped(r#"
						const link = document.querySelector('#fonts');
						link.onload = () => {
							link.onload = null;
							link.rel = 'stylesheet';
						};
					"#))
                }
                noscript {
                    link rel="stylesheet" href="https://use.typekit.net/gnn8txw.css" nonce=(nonce) {}
                }
                script src="https://cdnjs.cloudflare.com/ajax/libs/htmx/1.9.4/htmx.min.js" integrity="sha512-ZM2vxgVBxhBI5Etj/c/qcJV+upate3VzbVQOQRCx1YGuyEX9dYdMh8pRUot4xIwtAay6QwRQC/FdXRjSWIEHrg==" crossorigin="anonymous" referrerpolicy="no-referrer" nonce=(nonce) {}
                script nonce=(nonce) {
                    (PreEscaped(r#"
					htmx.config.includeIndicatorStyles = false;
					"#))
                }
            }
            body hx-boost="true" {
                header {
                    a href="/" { (PreEscaped(domain)) }
                }
                main {
                    (main)
                }
                footer {
                    a href="/privacy" { "Privacy Policy" }
                }
                noscript {
                    #noscript {
                        h1 { "You must enable Javascript in order to use this site" }
                    }
                }
            }
        }
    }
}

pub async fn get_root(req: Request, _: RouteContext<()>) -> Result<Response> {
    let baseurl = req.headers().get("host")?.ok_or("host header not set")?;
    let http_whitelist = vec!["localhost:8787", "127.0.0.1:8787"];

    // if localhost, http, otherwise, https
    let proto = if http_whitelist.contains(&baseurl.as_str()) {
        "http"
    } else {
        "https"
    };

    let url_str = format!("{}://{}/{}", proto, baseurl, "dubai");
    let url = Url::parse(&url_str).map_err(|e| format!("parse url: {}", e))?;

    Response::redirect_with_status(url, StatusCode::FOUND.as_u16())
}

fn generate_nonce() -> Result<String> {
    let mut buf = [0u8; 32];
    getrandom::getrandom(&mut buf).map_err(|e| format!("getrandom: {}", e))?;
    let nonce = base64::encode(&buf);
    Ok(nonce)
}

pub async fn get_privacy(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let nonce = generate_nonce()?;

    let main = html! {
        section #privacy {
            h1 { "Privacy Policy" }
            p { "Your privacy is important to us. It is getresidence.org's policy to respect your privacy regarding any information we may collect from you across our website, and other sites we own and operate." }
            p { "We only ask for personal information when we truly need it to provide a service to you. We collect it by fair and lawful means, with your knowledge and consent. We also let you know why we’re collecting it and how it will be used." }
            p { "We only retain collected information for as long as necessary to provide you with your requested service. What data we store, we’ll protect within commercially acceptable means to prevent loss and theft, as well as unauthorized access, disclosure, copying, use or modification." }
            p { "We don’t share any personally identifying information publicly or with third-parties, except when required to by law." }
            p { "Our website may link to external sites that are not operated by us. Please be aware that we have no control over the content and practices of these sites, and cannot accept responsibility or liability for their respective privacy policies." }
            p { "You are free to refuse our request for your personal information, with the understanding that we may be unable to provide you with some of your desired services." }
            p { "Your continued use of our website will be regarded as acceptance of our practices around privacy and personal information. If you have any questions about how we handle user data and personal information, feel free to contact us." }
            p { "This policy is effective as of 1 January 2023." }
        }
    };

    let markup = base_layout(
        "getresidence.org",
        "🔒",
        "Privacy Policy",
        "Your privacy is important to us. It is getresidence.org's policy to respect your privacy.",
        main,
        &nonce,
    );

    let headers = default_headers(Some(&nonce));
    Ok(Response::from_html(markup.into_string())?.with_headers(headers))
}

#[derive(Serialize, Deserialize)]
struct Name(String);

impl FromStr for Name {
    type Err = Error;

    fn from_str(name: &str) -> Result<Self> {
        ensure!(name.len() > 0, "name must not be empty");
        ensure!(name.len() < 100, "name must be less than 100 characters");
        for c in name.chars() {
            ensure!(
                c.is_alphanumeric() || c.is_whitespace(),
                "name must not contain special characters"
            );
        }
        Ok(Self(name.to_owned()))
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

type Email = EmailAddress;

#[derive(Serialize, Deserialize)]
struct Row {
    id: u64,
    name: Name,
    email: Email,
}

struct Database {
    d1: D1Database,
}

impl Database {
    fn new(d1: D1Database) -> Self {
        Self { d1 }
    }

    async fn get_row(&self, id: u64) -> Result<Row> {
        let statement = self.d1.prepare("SELECT * FROM sessions WHERE id = $1;");
        let query = statement.bind(&vec![JsValue::from_str(&id.to_string())])?;
        let row = query
            .first::<Row>(None)
            .await?
            .ok_or("get onboarding fields failed")?;

        Ok(row)
    }

    async fn set_name(&self, id: u64, validated: &Name) -> Result<()> {
        let statement = self
            .d1
            .prepare("UPDATE sessions SET name = ? WHERE id = ?;");
        let query = statement.bind(&vec![
            JsValue::from_str(validated.as_ref()),
            JsValue::from_str(&id.to_string()),
        ])?;
        let res = query.run().await?.results::<()>().err();
        if let Some(e) = res {
            return Err(format!("set name failed: {}", e).into());
        }
        Ok(())
    }

    async fn set_email(&self, id: u64, email: &Email) -> Result<()> {
        let statement = self
            .d1
            .prepare("UPDATE sessions SET email = ? WHERE id = ?;");
        let query = statement.bind(&vec![
            JsValue::from_str(email.as_str()),
            JsValue::from_str(&id.to_string()),
        ])?;
        let res = query.run().await?.results::<()>().err();
        if let Some(e) = res {
            return Err(format!("set email failed: {}", e).into());
        }
        Ok(())
    }

    async fn create_session(&self) -> Result<u64> {
        let query = self
            .d1
            .prepare("INSERT INTO sessions (name, email) VALUES ('', '') RETURNING id;");
        let id = query
            .first::<u64>(Some("id"))
            .await?
            .ok_or("create session failed")?;
        Ok(id)
    }
}

struct Auth {
    branca_key: Vec<u8>,
}

impl Auth {
    pub fn from_base64(key: &str) -> Result<Self> {
        let branca_key = base64::decode(key).map_err(|e| format!("base64 decode: {}", e))?;
        ensure!(branca_key.len() == 32, "branca key must be 32 bytes");

        Ok(Self { branca_key })
    }

    pub fn mint_token(&self, id: u64) -> Result<String> {
        let mut token = Branca::new(&self.branca_key).map_err(|e| format!("branca new: {}", e))?;
        let seconds = Date::now().as_millis() / 1000;
        token.set_timestamp(seconds as u32);
        let plaintext = id.to_le_bytes().to_vec();
        let token = token
            .encode(&plaintext)
            .map_err(|e| format!("branca encode: {}", e))?;
        Ok(token)
    }

    pub fn get_id(&self, session_token: &str) -> Result<u64> {
        let token = Branca::new(&self.branca_key).map_err(|e| format!("branca new: {}", e))?;
        let plaintext = token
            .decode(session_token, 0)
            .map_err(|e| format!("branca decode: {}", e))?;
        let id = u64::from_le_bytes(plaintext[..8].try_into().unwrap());
        Ok(id)
    }
}

pub async fn get_dubai(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let nonce = generate_nonce()?;

    let db = Database::new(ctx.env.d1("DB")?);
    let branca_key_base64 = ctx.secret("BRANCA_PRIVATE_KEY_BASE64")?.to_string();
    let auth = Auth::from_base64(&branca_key_base64)?;

    let (main, headers) = if let Ok(session) = get_session_value(&mut req) {
        let id = auth.get_id(&session)?;
        let row = db.get_row(id).await?;
        let main = html! {
            (dubai::main())
            (dubai::onboarding(row.name.as_ref(),"",""))
        };

        let headers = default_headers(Some(&nonce));
        (main, headers)
    } else {
        let id = db.create_session().await?;
        let token = auth.mint_token(id)?;
        let main = html! {
            (dubai::main())
            (dubai::onboarding("","",""))
        };

        let mut headers = default_headers(Some(&nonce));
        headers.append(
            "Set-Cookie",
            &format!("session={}; HttpOnly; SameSite", token),
        )?;
        (main, headers)
    };

    let markup = base_layout(
        "getresidence.org",
        "🇦🇪",
        "Get Legal Residency in Dubai",
        "Get Legal Residency in Dubai. Legally pay zero Taxes, or close to it!",
        main,
        &nonce,
    );

    Ok(Response::from_html(&markup.into_string())?.with_headers(headers))
}

fn get_session_value(req: &mut Request) -> Result<String> {
    let cookie = req.headers().get("Cookie")?.ok_or("cookie not set")?;
    let cookie = cookie.split(";").collect::<Vec<&str>>();
    let cookie = cookie
        .iter()
        .find(|x| x.contains("session="))
        .ok_or("session not found")?;
    let cookie = cookie.split("=").collect::<Vec<&str>>();
    let cookie = cookie.get(1).ok_or("session not found")?;
    let cookie = cookie.to_string();
    Ok(cookie)
}

pub async fn put_dubai_name(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let session = get_session_value(&mut req)?;
    let auth = Auth::from_base64(&ctx.secret("BRANCA_PRIVATE_KEY_BASE64")?.to_string())?;
    let id = auth.get_id(&session)?;

    let name = get_field(req.form_data().await?, "name")?;
    let markup = match Name::from_str(&name) {
        Ok(validated) => {
            let db = Database::new(ctx.env.d1("DB")?);
            db.set_name(id, &validated).await?;
            dubai::entry(
                "name",
                validated.as_ref(),
                dubai::EntryState::Valid,
                Some(format!("✅ name has been saved—{}", name.as_str())),
            )
        }
        Err(e) => dubai::entry(
            "name",
            &name,
            dubai::EntryState::Invalid,
            Some(format!("❌ {}", e)),
        ),
    };

    let headers = default_headers(None);
    Ok(Response::from_html(&markup.into_string())?.with_headers(headers))
}

pub async fn put_dubai_email(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let session = get_session_value(&mut req)?;
    let auth = Auth::from_base64(&ctx.secret("BRANCA_PRIVATE_KEY_BASE64")?.to_string())?;
    let id = auth.get_id(&session)?;

    let email = get_field(req.form_data().await?, "email")?;
    let markup = match Email::from_str(&email) {
        Ok(validated) => {
            let db = Database::new(ctx.env.d1("DB")?);
            db.set_email(id, &validated).await?;
            dubai::entry(
                "email",
                validated.as_ref(),
                dubai::EntryState::Valid,
                Some(format!("✅ email has been saved—{}", email.as_str())),
            )
        }
        Err(e) => dubai::entry(
            "email",
            &email,
            dubai::EntryState::Invalid,
            Some(format!("❌ {}", e)),
        ),
    };

    let headers = default_headers(None);
    Ok(Response::from_html(&markup.into_string())?.with_headers(headers))
}

pub async fn put_dubai_phone(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let session = get_session_value(&mut req)?;
    let d1 = ctx.env.d1("getresidence-org")?;

    let value = get_field(req.form_data().await?, "phone")?;
    Response::from_html(
        dubai::entry(
            "phone",
            &value,
            dubai::EntryState::Valid,
            Some(format!("✅ phone number has been saved—{}", value)),
        )
        .into_string(),
    )
}
