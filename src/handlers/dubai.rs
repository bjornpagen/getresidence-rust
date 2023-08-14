use maud::{html, Markup};

pub fn main() -> Markup {
    html! {
        section #hero {
            h1 {"Get Legal Residency in Dubai" }
            img src="https://imagedelivery.net/DYW9P4DJWiLboxGcOOuHaQ/fecc28ed-93fc-4498-f20b-364d27290100/public" alt="Dubai" {}
            p { "Legally pay zero Taxes, or close to it!" }
            p { "Get legal residency and a corporation..." }
            p { "In one of the only zero tax countries in the world!" }
            p { "Just experience Dubai, and leave with your legal residency card and a Dubai corporation." }
            a .btn1 href="#onboarding" {
                span { "Register Now" }
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
            a .btn1 href="#onboarding" {
                span { "Register Now" }
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
            a .btn1 href="#onboarding" {
                span { "Register Now" }
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
            a .btn1 href="#onboarding" {
                span { "Register Now" }
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
            a .btn1 href="#onboarding" {
                span { "Register Now" }
            }
        }
    }
}

pub fn onboarding(name: &str, email: &str, phone: &str) -> Markup {
    html! {
        section #onboarding {
            h2 { "Please fill out the following form to get started." }
            (entry("name", name, EntrySchema::Text, EntryState::Init, None))
            (entry("email", email, EntrySchema::Email, EntryState::Init, None))
            (entry("phone", phone, EntrySchema::Tel, EntryState::Init, None))
            button .btn1 {
                span { "Submit" }
            }
        }
    }
}

pub enum EntryState {
    Init,
    Valid,
    Invalid,
}

pub enum EntrySchema {
    Text,
    Email,
    Tel,
}

pub fn entry(
    name: &str,
    value: &str,
    schema: EntrySchema,
    state: EntryState,
    small: Option<String>,
) -> Markup {
    let class = match state {
        EntryState::Init => "",
        EntryState::Valid => "valid",
        EntryState::Invalid => "invalid",
    };

    let schema = match schema {
        EntrySchema::Text => "text",
        EntrySchema::Email => "email",
        EntrySchema::Tel => "tel",
    };

    html! {
        #entry .(class) {
            label for=(name) { (name) }
            input #(name) type=(schema) name=(name) required="required" value=(value)
                hx-put=(format!("/dubai/{}", name)) hx-swap="outerHTML" hx-sync="input:queue" hx-target="closest #entry" {}
            @if let Some(small) = small {
                small { (small) }
            }
        }
    }
}
