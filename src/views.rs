use maud::{html, Markup, PreEscaped, DOCTYPE};

use crate::{db::{Account, Transaction}, helpers::get_total_as_formatted_string};

pub fn simple_error(message: &str) -> Markup {
    html! {
        p { (message) }
        a href="/logout" { "You could try logging out." }
    }
}

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

fn account(id: i32, name: &str, total: &str) -> Markup {
    html! {
        a hx-get=(format!("/accounts/{}", id)) hx-target="#content" hx-swap="innerHTML" class="block text-sm flex justify-between py-1 px-3" {
            p class="tracking-wide text-sm" { (name) }
            p { (total) }
        }
    }
}

pub fn transactions_list(transactions: Vec<Transaction>) -> Markup {
    html! {
        div class="block w-full grid grid grid-cols-6" {
            div { "Id" }
            div { "Memo" }
            div { "Date" }
            div { "Cleared" }
            div { "Inflow" }
            div { "Outflow" }
            @for transaction in transactions {
                div { (transaction.id) }
                div { (transaction.memo) }
                div { (transaction.date) }
                div { (transaction.cleared) }
                div { (get_total_as_formatted_string(transaction.inflow)) }
                div { (get_total_as_formatted_string(transaction.outflow)) }
            }
        }
    }
}

fn no_account() -> Markup {
    html! {
        div class="w-full rounded bg-gray-800 p-2 text-left" {
            h5 { "No Accounts" }
            p class="text-sm" { "You can't budget without adding accounts to YMNAB first. How about adding one now?" }
        } 
    }
}

fn create_new_account() -> Markup {
    html! {
        button onclick="openAccountForm()" class="rounded bg-gray-800 hover:bg-gray-700 transition-colors py-1 px-2 text-left" { "Add Account" }
        form id="new-account-form" class="hidden space-y-2" hx-post="/account/create" {
            input class="w-full rounded bg-gray-800 border border-gray-700 py-1 px-2" type="text" name="name" placeholder="Nickname" {}
            select class="w-full rounded bg-gray-800 border border-gray-700 py-1 px-2" type="text" name="type" placeholder="Type" {
                option value="checking" { "Checking" }
            }
            input class="w-full rounded bg-gray-800 border border-gray-700 py-1 px-2" type="text" name="starting_balance" placeholder="Starting balance" {}
            button type="submit" class="rounded bg-gray-800 hover:bg-gray-700 transition-colors py-1 px-2 text-left" { "Save" }
        }
        script {
            (PreEscaped(r#"
                function openAccountForm() {
                    const form = document.getElementById("new-account-form");
                    form.classList.remove("hidden")
                }
            "#))
        }
    }
}

pub fn accounts_partial(accounts: Vec<Account>, budget_total: String) -> Markup {
    html! {
        div hx-trigger="accountsUpdated" hx-get="/api/accounts" hx-swap="outerHTML" class="w-full space-y-2" {
            @if accounts.is_empty() {
                (no_account())
            } @else {
                div class="flex justify-between py-1 px-3" {
                    p class="tracking-wide uppercase" { "Budget" }
                    p class="tracking-wide uppercase text-sm" { (budget_total) }
                }
                @for acc in accounts {
                    (account(acc.id, &acc.name, &acc.get_total_as_formatted_string()))
                }
            }
            (create_new_account())
        }
    }
}

pub fn home(accounts: Vec<Account>, budget_total: String) -> Markup {
    let title: &str = "Home";
    html! {
        (header(&title))
        body class="w-full min-h-screen" {
            main class="grid grid-cols-5 bg-gray-950" {
                nav class="col-span-1 bg-gray-950 text-white h-screen flex-col items-center text-left justify-center p-2" {
                    a class="w-full rounded block py-1 px-3 bg-blue-800" href="/" { "Home" }
                    a class="w-full rounded block py-1 px-3" href="/" { "All Accounts" }
                    (accounts_partial(accounts, budget_total))
                }
                section id="content" class="col-span-4 bg-gray-900 text-white" {
                    p { "Content" }
                }
            }
        }
    }

}

pub fn error_message(error: &str) -> Markup {
    html! {
        p class="text-red-800 font-semibold" { (error) }
    }
}

pub fn signup() -> Markup {
    let title: &str = "Sign up";
    html! {
        (header(&title))
        body hx-boost="true" class="container mx-auto" {
            h1 { (&title) }
            div class="grid grid-cols-2 gap-4 pt-12 container mx-auto w-3/5" {
                div {
                    p class="font-semibold" { "Create the life you want with YMNAB" }
                }
                div class="text-center space-y-4" {
                    h2 class="text-indigo-600 text-4xl" { "Log In" }
                    p { "Have an account? " a href="/login" class="text-blue-500" { "Log in." } }
                    form hx-post="/signup" hx-target="#error" class="flex flex-col space-y-4" {
                        input name="name" placeholder="Name" class="p-3 rounded-sm border" type="text";
                        input name="email" placeholder="Email Address" class="p-3 rounded-sm border" type="email";
                        input name="password" placeholder="Password" class="p-3 rounded border" type="password";
                        div id="error" {}
                        button type="submit" class="text-center text-white font-semibold rounded bg-indigo-600 p-3" { "Sign Up" }
                    }
                }
            }
            (footer())
        }
    }
}

pub fn login() -> Markup {
    let title: &str = "YMNAB";
    html! {
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

