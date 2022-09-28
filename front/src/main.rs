use yew::prelude::*;
use reqwasm::http::Request;
use web_sys::HtmlInputElement;
use js_sys::JsString;

fn main() {
    yew::start_app::<Login>();
}

#[derive(PartialEq, Debug, Clone)]
struct Account {
    id : String,
    pw : String    
}

#[function_component(Login)]
fn login() -> Html{
    let input_id_ref = NodeRef::default();
    let input_pw_ref = NodeRef::default();

    // document node reference들은 반드시 clone해서 써야 한다
    let input_id = input_id_ref.clone();
    let input_pw = input_pw_ref.clone();

    // https://yew.rs/docs/concepts/function-components/pre-defined-hooks#use_state
    let login_result = use_state(|| None);
    let _login_result = login_result.clone();
    // login() 내의 모든 변수들은 Callback::from() Closure로 소유권이 move 됨
    let onclick = Callback::from(move |_| {
            let id = input_id.cast::<HtmlInputElement>().unwrap().value();
            let pw = input_pw.cast::<HtmlInputElement>().unwrap().value();
            let login_result = login_result.clone();
            // Callback::from() Closure 내의 모든 변수들은  wasm_bindgen_futures::spawn_local() Closure로 소유권이 move 됨
            wasm_bindgen_futures::spawn_local(async move {
                let backend_url = format!("http://localhost:8081/login?id={id}&pw={pw}");
                let backend_msg = Request::get(&backend_url).send().await.unwrap().text().await.unwrap();                
                web_sys::console::log_1(&JsString::from(backend_msg.clone()));
                if backend_msg == String::from("welcome!") {
                    login_result.set(Some(true));
                }
                else {
                    login_result.set(Some(false));
                }
            });
        });

    // 따라서 아래에서 login() 내의 변수들을 사용하려면 clone한 것을 사용해야 함
    html!{
        <div>
            <p>{"id"}</p>
            <input ref = {input_id_ref} type="text"/>
            <p>{"password"}</p>
            <input ref = {input_pw_ref} type="text"/>
            <button onclick={onclick}>{"login"}</button>
            // UseStateHandle<>은 Deref traits를 구현하고 있기 때문에 담겨있는 실제 state 값을 참조하려면 역참조 연산자 *를 사용해야 함
            <LoginResult login_result={*_login_result} />
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct LoginResultProps {
    login_result: Option<bool>
}

#[function_component(LoginResult)]
// prop은 LoginResultProps type의 login_result를 borrow함
fn login_result(props : &LoginResultProps) -> Html{
    let login_result_msg = match &props.login_result {
        Some(true) => "login success!!!",
        Some(false) => "login fail...",
        None => ""
    };

    html!{
        <div>{login_result_msg}</div>
    }
}
