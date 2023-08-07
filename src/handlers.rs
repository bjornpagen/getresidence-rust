use http::StatusCode;
use maud::{html, PreEscaped, DOCTYPE};
use worker::*;

const STYLE_RESET: &'static str = r#"*,::after,::before{box-sizing:border-box;border-width:0;border-style:solid;border-color:#e5e7eb}html{line-height:1.5;-webkit-text-size-adjust:100%;-moz-tab-size:4;tab-size:4;font-family:ui-sans-serif,system-ui,-apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,"Helvetica Neue",Arial,"Noto Sans",sans-serif,"Apple Color Emoji","Segoe UI Emoji","Segoe UI Symbol","Noto Color Emoji"}body{margin:0;line-height:inherit}hr{height:0;color:inherit;border-top-width:1px}abbr:where([title]){text-decoration:underline dotted}h1,h2,h3,h4,h5,h6{font-size:inherit;font-weight:inherit}a{color:inherit;text-decoration:inherit}b,strong{font-weight:bolder}code,kbd,pre,samp{font-family:ui-monospace,SFMono-Regular,Menlo,Monaco,Consolas,"Liberation Mono","Courier New",monospace;font-size:1em}small{font-size:80%}sub,sup{font-size:75%;line-height:0;position:relative;vertical-align:baseline}sub{bottom:-.25em}sup{top:-.5em}table{text-indent:0;border-color:inherit;border-collapse:collapse}button,input,optgroup,select,textarea{font-family:inherit;font-feature-settings:inherit;font-variation-settings:inherit;font-size:100%;font-weight:inherit;line-height:inherit;color:inherit;margin:0;padding:0}button,select{text-transform:none}[type=button],[type=reset],[type=submit],button{-webkit-appearance:button;background-color:transparent;background-image:none}:-moz-focusring{outline:auto}:-moz-ui-invalid{box-shadow:none}progress{vertical-align:baseline}::-webkit-inner-spin-button,::-webkit-outer-spin-button{height:auto}[type=search]{-webkit-appearance:textfield;outline-offset:-2px}::-webkit-search-decoration{-webkit-appearance:none}::-webkit-file-upload-button{-webkit-appearance:button;font:inherit}summary{display:list-item}blockquote,dd,dl,figure,h1,h2,h3,h4,h5,h6,hr,p,pre{margin:0}fieldset{margin:0;padding:0}legend{padding:0}menu,ol,ul{list-style:none;margin:0;padding:0}textarea{resize:vertical}input::placeholder,textarea::placeholder{opacity:1;color:#9ca3af}[role=button],button{cursor:pointer}:disabled{cursor:default}audio,canvas,embed,iframe,img,object,svg,video{display:block;vertical-align:middle}img,video{max-width:100%;height:auto}[hidden]{display:none}"#;

const STYLE_SYSTEM: &'static str = r#"
:root {
	--font-emoji: "Apple Color Emoji","Segoe UI Emoji","Segoe UI Symbol","Noto Color Emoji";
	--font-text: "neue-haas-grotesk-text", "SF Pro Text", system-ui, sans-serif, var(--font-emoji);
	--font-display: "neue-haas-grotesk-display", "SF Pro Display", system-ui, sans-serif, var(--font-emoji);
	--row-height: 24px;
	--max-width: calc(var(--row-height) * 28);
}
body {
	-webkit-font-smoothing: antialiased;
	-moz-osx-font-smoothing: grayscale;

	padding-top: var(--row-height);
	padding-bottom: calc(var(--row-height) * 4);
	padding-left: calc(var(--row-height) / 2);
	padding-right: calc(var(--row-height) / 2);

	max-width: var(--max-width);
	margin-left: auto;
	margin-right: auto;

	display: flex;
	flex-direction: column;
	gap: calc(var(--row-height) * 2);

	font-size: 18px;
	line-height: var(--row-height);
	font-family: var(--font-text);
	font-weight: 400;
}
h1 {
	font-size: 42px;
	line-height: calc(var(--row-height) * 2);
	font-family: var(--font-display);
	font-weight: 600;
	font-style: normal;
	text-wrap: balance;
}
h2 {
	font-weight: 700;
}
.btn1 {
	display: flex;
	height: calc(var(--row-height) * 2);
	justify-content: center;
	align-items: center;
	align-self: stretch;
	background: #000;
	width: 100%;
}
.btn1 > span {
	color: #FFF;
	text-align: center;
	font-family: var(--font-text);
	font-size: 18px;
	font-style: normal;
	font-weight: 700;
	line-height: var(--row-height);
}
@media (min-width: 528px) {
	.btn1 {
		grid-column: 2 / span 2;
	}
}
dl {
	display: grid;
	grid-template-columns: 1fr 1fr 1fr 1fr;
	column-gap: calc(var(--row-height) / 2);
	row-gap: var(--row-height);
}
dl > dt {
	font-style: italic;
	grid-column: 1 / span 3;
}
dl > dd {
	grid-column: 2 / span 3;
}
ol > li {
	display: grid;
	grid-template-columns: 1fr 1fr 1fr 1fr;
	column-gap: calc(var(--row-height) / 2);
	row-gap: var(--row-height);
}
li {
	counter-increment: my-counter;
}
li::before {
	content: counter(my-counter) ". ";
	grid-column: 1;
}
section {
	display: grid;
	grid-template-columns: 1fr 1fr 1fr 1fr;
	column-gap: calc(var(--row-height) / 2);
	row-gap: var(--row-height);
}
section > * {
	grid-column: 1 / span 4;
}
@media (min-width: 528px) {
	section {
		> h1, > h2, > h3, > h4, > h5, > h6 {
			grid-column: 1 / span 3;
		}
		> p {
			grid-column: 2 / span 3;
		}
	}
}
ol {
	display: flex;
	flex-direction: column;
	gap: var(--row-height);
}
ol > li > p {
	grid-column: 2 / span 3;
}
form {
	display: grid;
	grid-template-columns: 1fr 1fr 1fr 1fr;
	column-gap: calc(var(--row-height) / 2);
	row-gap: 0;
}
form > label {
	font-weight: 700;
	grid-column: 1 / span 4;
	margin-bottom: calc(var(--row-height) / 2);
}
form > input, form > select, form > textarea {
	height: calc(var(--row-height) * 1.5);
	border: 2px solid #000;
	grid-column: 1 / span 4;
	margin-bottom: calc(var(--row-height));
	padding-left: calc(var(--row-height) / 4);
}
form > *:last-child {
	margin-bottom: 0;
}
@media (min-width: 528px) {
	form > label {
		grid-column: 1;
		margin-bottom: 0;
	}
	form > input, form > select, form > textarea {
		grid-column: 2 / span 3;
		margin-bottom: calc(var(--row-height) * 1.5);
	}
}
"#;

// base_layout is the base HTML template we will use for all responses.
pub fn base_layout(
    domain: &'static str,
    title: &'static str,
    description: &'static str,
    body: PreEscaped<String>,
    styles: &'static str,
) -> PreEscaped<String> {
    html! {
        (DOCTYPE)
        meta charset="utf-8" {}
        html lang="en" {
            head {
                title { (PreEscaped(title)) }
                meta name="description" content=(PreEscaped(description)) {}
                meta name="viewport" content="width=device-width, initial-scale=1" {}
                link rel="preload" href="https://use.typekit.net/gnn8txw.css" as="style" onload="this.onload=null;this.rel='stylesheet'" {}
                noscript {
                    link rel="stylesheet" href="https://use.typekit.net/gnn8txw.css" {}
                }
                style #reset {
                    (PreEscaped(STYLE_RESET))
                }
                style #global {
                    (PreEscaped(STYLE_SYSTEM))
                }
                style #local {
                    (PreEscaped(styles))
                }
            }
            body {
                header {
                    p { (PreEscaped(domain)) }
                }
                (body)
                footer {
                    p { "© 2023 " (PreEscaped(domain)) }
                }
            }
        }
    }
}

pub async fn dubai_handler(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let body = html! {
        section #hero {
            h1 {"Get Legal Residency in Dubai." }
            img src="https://imagedelivery.net/DYW9P4DJWiLboxGcOOuHaQ/fecc28ed-93fc-4498-f20b-364d27290100/public" alt="Dubai" {}
            p { "Legally pay zero Taxes, or close to it!" }
            p { "Get legal residency and a corporation..." }
            p { "In one of the only zero tax countries in the world!" }
            p { "Just experience Dubai, and leave with your legal residency card and a Dubai corporation." }
            a .btn1 href="/dubai/onboarding" {
                span { "Register Now." }
            }
        }
        section #perks {
            h2 { "Here's Everything You Get:" }
            dl {
                dt { "All Legal Fees" }
                dd { "We coordinate with the attorneys and government offices and take care of all the legal fees and document fees." }
                dt { "Document Prep" }
                dd { "We prep all of your needed documents for the UAE government and submit them all so you don't have to worry about a thing." }
                dt { "Save Taxes In Dubai Seminar" }
                dd { "We host a seminar where you will learn all the ins and outs of saving massive amounts of taxes by incorporating in Dubai." }
                dt { "Flight and Hotel Booking Assistance" }
                dd { "We will help you book your flight and hotel as well as find a good hotel for you based on your tastes and budget." }
                dt { "Round-The-Clock On-Site Support" }
                dd { "You'll have access to our team during your entire stay, your resource for anything you need and any questions you have." }
                dt { "Transportation To All Appointments" }
                dd { "We will drive you to all the necessary appointments and show you exactly what you need to do." }
            }
            a .btn1 href="/dubai/onboarding" {
                span { "Register Now." }
            }
        }
        section #info {
            p {
                "Save "
                em { "massive" }
                " money on taxes."
            }
            p { "Protect your business and your assets." }
            p { "Diversify your international life." }
            p { "Improve your lifestyle." }
            p { "This is the fastest and easiest way to get full legal residency and a corporation in the United Arab Emirates (UAE) of which Dubai is its most popular city." }
            p {
                "The UAE has "
                em { "no" }
                " corporate tax, "
                em { "no" }
                " income tax, "
                em { "no" }
                " payroll taxes, and "
                em { "no" }
                " capital gains taxes! It is a massively booming economy and one of the fastest growing countries in the world."
            }
            p { "It is also one of the most friendly countries to Westerners and foreigners and one of the best and easiest places to get legal residency." }
            p {
                "Sign up now, and join us in November, have a fun and exciting adventure, and let us do all the hard work for you. You just come to Dubai and we take care of "
                em { "everything else." }
            }
            p { "You will actually leave Dubai with your residency card and your fully-formed Free Zone, zero-tax corporation!" }
            a .btn1 href="/dubai/onboarding" {
                span { "Register Now." }
            }
        }
        section #how-it-works {
            h2 { "How it Works:" }
            ol {
                li {
                    p { "You sign up as soon as you can or schedule a call with us if you have questions, using the register buttons above or below." }
                }
                li {
                    p { "We send you a detailed questionnaire that you fill out and return to us so we can start processing your residency." }
                }
                li {
                    p { "With our assistance, you book your flight to Dubai, UAE and plan on being there from November 2nd 2023 to November 15th 2023." }
                }
                li {
                    p { "With our assistance, you'll book your hotel in Dubai. There are hundreds of great hotels in Dubai for all tastes and budgets. We will assist you in picking the best hotel for you." }
                }
                li {
                    p { "You submit your documents to us and we will process them with the UAE government and residency attorneys." }
                }
                li {
                    p { "During your 14-day stay, we will drive you to all your appointments to get your residency and your Dubai corporation and tell you exactly what to do once you get there. We take care of all legal and government fees." }
                }
                li {
                    p { "Once you leave on November 15th, you will have your residency card in your hand plus your fully formed Dubai Free Zone corporation. And now you save thousands upon thousands of dollars in taxes, legally!" }
                }
            }
            a .btn1 href="/dubai/onboarding" {
                span { "Register Now." }
            }
        }
        section #faq {
            h2 { "Frequently Asked Questions:" }
            dl {
                dt { "Are the cost of this service and my travel costs tax deductible?" }
                dd { "The answer is, “yes, most likely, but check with your accountant.” You're coming to Dubai and using this service to start a corporation and get residency, which is required for a corporation, as well as attend business/tax seminars. This means that the cost of this service and your travel costs should be tax deductible. But we can't give tax advice so please check with your tax professional." }
                dt { "When is the event?" }
                dd { "November 2nd 2023 to November 15th 2023. You need to be there that long to ensure you get your residency. Of course you are welcome to come earlier or stay later! Dubai is a fantastic place!" }
                dt { "When is the deadline for me to sign up?" }
                dd { "The price goes up by $500 on August 31st 2023. Your absolute deadline to sign up is October 10th 2023. After that, we can not take anyone else since we might not be able to get your documents prepared in time." }
                dt { "If I don't want to move out of my country, can I still have money on taxes by having a Dubai corporation? Even if I'm an American?" }
                dd { "Yes and yes. You won't save as much taxes than if you actually left your high-taxed country, but yes, you can still save some money in taxes even if you don't move away." }
                dt { "Do I just stop paying taxes in my home country if I move to Dubai?" }
                dd { "If you're not an American, yes, it's that simple. If you are an American, your first $120,000 per year earned will be tax-free and there are other ways of saving taxes beyond that if you're willing to get creative." }
                dt { "Do I have to live in Dubai/UAE or spend lots of time there to maintain my residency?" }
                dd { "No! All you need to do is visit Dubai once every six months. That's it. You can arrive and then leave the same day. Many people just use Dubai as a layover on their way to fly somewhere else and as long as you exit the airport and go back in, that's all you need to do." }
                dt { "How long does it take to get my residency?" }
                dd { "You'll have your residency card in your hand when you leave after your 2-week stay. Residency in Dubai is one of the fastest in the world." }
                dt { "How long does my residency last?" }
                dd { "2 years. After that you can renew for longer. You will also (likely) have to renew your corporation every year as well and there are annual fees associated with this." }
                dt { "Will I be able to open a bank account once I get my residency?" }
                dd { "Yes. You will be able to set up a personal checking/savings account once your residency is done. However, Dubai is a little slow setting up bank accounts and it takes 2-3 weeks to do so, so you'll either need to extend your stay or return to Dubai for a second visit to get this done." }
                dt { "What if something comes up in my schedule and I have to cancel my trip?" }
                dd { "No problem. If you have to cancel your November trip you can simply reschedule your trip for a later date, go to Dubai then, and we'll support you over the phone (instead of in-person) and coordinate with the attorneys remotely. Things will be slightly less convenient but we'll still make it work." }
            }
            a .btn1 href="/dubai/onboarding" {
                span { "Register Now." }
            }
        }
    };

    const STYLES_LOCAL: &'static str = r#"
	#hero img {
		height: calc(var(--row-height) * 8);
		width: 100%;
		object-fit: cover;
		object-position: center;
	}
	"#;

    let body = base_layout(
        "getresidence.org",
        "Get Legal Residency in Dubai",
        "Get Legal Residency in Dubai. Legally pay zero Taxes, or close to it!",
        body,
        STYLES_LOCAL,
    );

    Response::from_html(body.into_string())
}

pub async fn dubai_onboarding_handler(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let body = html! {
        section #form {
            h1 { "Dubai Residence Onboarding." }
            img src="https://imagedelivery.net/DYW9P4DJWiLboxGcOOuHaQ/fecc28ed-93fc-4498-f20b-364d27290100/public" alt="Dubai" {}
            p { "Please fill out the following form to get started." }
            form #submit method="POST" action="/api/dubai/onboarding" {
                label for="name" { "Name" }
                input type="text" name="name" id="name" required="required" {}
                label for="email" { "Email" }
                input type="email" name="email" id="email" required="required" {}
                label for="phone" { "Phone" }
                input type="tel" name="phone" id="phone" required="required" {}
                label for="country" { "Country" }
                input type="text" name="country" id="country" required="required" {}
            }
            button .btn1 type="submit" form="submit" {
                span { "Submit" }
            }
        }
    };

    const STYLES_LOCAL: &'static str = r#"
	#form img {
		height: calc(var(--row-height) * 8);
		width: 100%;
		object-fit: cover;
		object-position: center;
	}
	"#;

    let body = base_layout(
        "getresidence.org",
        "Get Legal Residency in Dubai",
        "Get Legal Residency in Dubai. Legally pay zero Taxes, or close to it!",
        body,
        STYLES_LOCAL,
    );

    Response::from_html(body.into_string())
}

pub async fn redirect_dubai(req: Request, _: RouteContext<()>) -> Result<Response> {
    let baseurl = req
        .headers()
        .get("host")
        .unwrap()
        .to_owned()
        .expect("get host header");

    // if localhost, http, otherwise, https
    let proto = if baseurl.eq("localhost:8787") {
        "http"
    } else {
        "https"
    };

    let url_str = format!("{}://{}/{}", proto, baseurl, "dubai");

    let url: Url = Url::parse(&url_str).unwrap();

    Response::redirect_with_status(url, StatusCode::FOUND.as_u16())
}
