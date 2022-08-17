use yew::{prelude::*, props};
use serde_json::Value;
use web_sys::{HtmlInputElement, Text};

fn main() {
    yew::start_app::<Login>();
}

#[derive(PartialEq, Debug, Clone)]
struct Account{
    id : String,
    pw : String,
}

#[function_component(Login)]
fn login() -> Html{
    let id = "ID";
    let pw = "PASSWORD";

    let id_input_ref = NodeRef::default();
    let id_input_ref_outer = id_input_ref.clone();
    let pw_input_ref = NodeRef::default();
    let pw_input_ref_outer = pw_input_ref.clone();

    let account_state = use_state_eq::<Option<Account>, _>( || None);
    let account_state_outer = account_state.clone();
    let login_state = use_state_eq::<Option<Result>, _>( || None);
    let login_state_outer = login_state.clone();

    let onclick = Callback::from(move |mouse_event:MouseEvent| {
        web_sys::console::log_1(&mouse_event.into());

        let id_input = id_input_ref.cast::<HtmlInputElement>().unwrap();
        let pw_input = pw_input_ref.cast::<HtmlInputElement>().unwrap();
        
        let id_input_v = id_input.value();
        let pw_input_v = pw_input.value();
        
        account_state.set(None);

        let account_state = account_state.clone();

        let test = Account {
            id : id_input_v,
            pw : pw_input_v,
        };
        account_state.set(Some(test));
    });

    html!{
        <div>
            <p>{id}</p>
            <input ref = {id_input_ref_outer.clone()} type="text"/>
            <p>{pw}</p>
            <input ref = {pw_input_ref_outer.clone()} type="text"/>
            <button {onclick}>{"LOGIN"}</button>
            <ResultComp account = {(*account_state_outer).clone()} login_state={(login_state_outer).clone()} />
        </div>
    }
}

#[derive(PartialEq, Debug, Clone)]
enum Result{
    Correct,
    Incorrect,
}

#[derive(Properties, PartialEq)]
struct ResultPros{
    account : Option<Account>,
    login_state : UseStateHandle<Option<Result>>,
}

#[function_component(ResultComp)]
fn resultcomp(props : &ResultPros) -> Html{

    let account = match &props.account{
        Some(p) => p,
        None => return html!{},
    };

    let login_state_outer = props.login_state.clone();
    let login_state = props.login_state.clone();

    let id = account.id.clone();
    let pw = account.pw.clone();

    let remote = Account{
        id : String::from("HoneyPot"),
        pw : String::from("1234"),
    };

    if id == remote.id && pw == remote.pw {
        login_state.set(Some(Result::Correct));
        web_sys::console::log_1(&"로그인 성공!!!".into());
    }else {
        login_state.set(Some(Result::Incorrect));  
        web_sys::console::log_1(&"로그인 실패...".into());          
    }

    html!{
        <div>
            <Connection result = {(*login_state_outer).clone()} />
        </div>
    }
}


#[derive(Properties, PartialEq)]
struct LoginResult{
    result : Option<Result>,
}

#[function_component(Connection)]
fn connection(props : &LoginResult) -> Html{
    let text = match &props.result{
        None => return html!{<div>{"로그인 해주세요."} </div>},
        Some(Result::Correct) => "로그인 성공!!!",
        Some(Result::Incorrect) => "로그인 실패...",
    };

    html!{
        <div>{text}</div>
    }
}