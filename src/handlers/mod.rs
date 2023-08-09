use http::StatusCode;
use maud::{html, PreEscaped, DOCTYPE};
use worker::*;

mod dubai;
mod styles;

fn emoji_to_favicon(emoji: &str) -> String {
    format!("data:image/svg+xml,<svg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%220 0 100 100%22><text y=%22.9em%22 font-size=%2290%22>{}</text></svg>", emoji)
}

fn field(data: FormData, field: &str) -> Result<String> {
    let name_entry = data.get(field).ok_or("field does not exist")?;

    match name_entry {
        FormEntry::Field(n) => Ok(n),
        _ => Err("expected a field".into()),
    }
}

// base_layout is the base HTML template we will use for all responses.
pub fn base_layout(
    domain: &'static str,
    icon: &'static str,
    title: &'static str,
    description: &'static str,
    body: PreEscaped<String>,
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
                style #reset {
                    (PreEscaped(styles::RESET))
                }
                style #global {
                    (PreEscaped(styles::SYSTEM))
                }
                link rel="preload" href="https://use.typekit.net/gnn8txw.css" as="style" onload="this.onload=null;this.rel='stylesheet'" {}
                noscript {
                    link rel="stylesheet" href="https://use.typekit.net/gnn8txw.css" {}
                }
                script src="https://cdnjs.cloudflare.com/ajax/libs/htmx/1.9.4/htmx.min.js" integrity="sha512-ZM2vxgVBxhBI5Etj/c/qcJV+upate3VzbVQOQRCx1YGuyEX9dYdMh8pRUot4xIwtAay6QwRQC/FdXRjSWIEHrg==" crossorigin="anonymous" referrerpolicy="no-referrer" {}
            }
            body hx-boost="true" {
                header {
                    a href="/" { (PreEscaped(domain)) }
                }
                main {
                    (body)
                }
                footer {
                    a href="/privacy" { "Privacy Policy" }
                }
                noscript {
                    #noscript {
                        h1 { "You must enable Javascript in order to use this site." }
                    }
                }
            }
        }
    }
}

pub async fn get_root(req: Request, _: RouteContext<()>) -> Result<Response> {
    let baseurl = req.headers().get("host")?.ok_or("get baseurl")?;
    let http_whitelist = vec!["localhost:8787", "127.0.0.1:8787"];

    // if localhost, http, otherwise, https
    let proto = if http_whitelist.contains(&baseurl.as_str()) {
        "http"
    } else {
        "https"
    };

    let url_str = format!("{}://{}/{}", proto, baseurl, "dubai");
    let url = Url::parse(&url_str).unwrap();

    Response::redirect_with_status(url, StatusCode::FOUND.as_u16())
}

pub async fn get_privacy(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let body = html! {
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

    let body = base_layout(
        "getresidence.org",
        "🔒",
        "Privacy Policy",
        "Your privacy is important to us. It is getresidence.org's policy to respect your privacy.",
        body,
    );

    Response::from_html(body.into_string())
}

pub async fn get_dubai(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let main = html! {
        (dubai::main())
        (dubai::onboarding("pillage","",""))
    };

    let markup = base_layout(
        "getresidence.org",
        "🇦🇪",
        "Get Legal Residency in Dubai",
        "Get Legal Residency in Dubai. Legally pay zero Taxes, or close to it!",
        main,
    );

    Response::from_html(markup.into_string())
}

pub async fn put_dubai_name(mut req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let value = field(req.form_data().await?, "name")?;
    Response::from_html(
        dubai::entry(
            "name",
            &value,
            dubai::EntryState::Valid,
            Some(format!("✅ name has been saved—{}", value)),
        )
        .into_string(),
    )
}

pub async fn put_dubai_email(mut req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let value = field(req.form_data().await?, "email")?;
    Response::from_html(
        dubai::entry(
            "email",
            &value,
            dubai::EntryState::Invalid,
            Some(format!(
                "❌ email already associated with an account—{}",
                value
            )),
        )
        .into_string(),
    )
}

pub async fn put_dubai_phone(mut req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let value = field(req.form_data().await?, "phone")?;
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

// pub async fn dubai_onboarding_handler(
//     mut req: Request,
//     _ctx: RouteContext<()>,
// ) -> Result<Response> {
//     let payload = html! {
//         p { (req.headers().values().join(" ")) }
//     };

//     let cookie = format!("name={};SameSite=strict;HttpOnly;Secure", "pig");

//     let mut headers = Headers::new();
//     headers.append("Set-Cookie", &cookie)?;

//     Ok(Response::from_html(payload.into_string())?.with_headers(headers))
// }
