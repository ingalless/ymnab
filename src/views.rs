use maud::{DOCTYPE, html, Markup};

/// A basic header with a dynamic `page_title`.
fn header(page_title: &str) -> Markup {
    html! {
        (DOCTYPE)
        meta charset="utf-8";
        title { (page_title) }
        script src="https://cdn.tailwindcss.com" {} 
        script src="https://unpkg.com/htmx.org@1.9.10" integrity="sha384-D1Kt99CQMDuVetoL1lrYwg5t+9QdHe7NLX/SoJYkXDFfX37iInKRy5xLSi8nO7UC" crossorigin="anonymous" {}
    }
}

/// A static footer.
fn footer() -> Markup {
    html! {
        footer { }
    }
}

pub fn home() -> Markup {
    let title: &str = "Home";
    html! {
        (header(&title))
        p { "Home" }
    }

}

pub fn login_error() -> Markup {
    html! {
        p class="text-red-800 font-semibold" { "User not found" }
    }
}

pub fn login() -> Markup {
    let title: &str = "YMNAB";
    html! {
        // Add the header markup to the page
        (header(&title))
        body hx-boost="true" class="container mx-auto" {
            h1 { (&title) }
            div class="grid grid-cols-2 gap-4 pt-12 container mx-auto w-3/5" {
                div {
                    p class="font-semibold" { "Create the life you want with YMNAB" }
                }
                div class="text-center space-y-4" {
                    h2 class="text-indigo-600 text-4xl" { "Log In" }
                    p { "New to YMNAB? " a href="/signup" class="text-blue-500" { "Sign up today." } }
                    form hx-post="/login" hx-target="#error" class="flex flex-col space-y-4" {
                        input name="email" placeholder="Email Address" class="p-3 rounded-sm border" type="email";
                        input name="password" placeholder="Password" class="p-3 rounded border" type="password";
                        div class="flex items-center justify-between" {
                            label for="remember" { input name="remember" type="checkbox" class="mr-1"; "Keep me logged in" }
                            a class="text-blue-600" { "Forgot password?" }
                        }
                        div id="error" {}
                        button type="submit" class="text-center text-white font-semibold rounded bg-indigo-600 p-3" { "Log In" }
                    }
                }
            }
            (footer())
        }
    }
}

