mod core;








use crate::core::BrainFuck;

use dioxus::{prelude::*, desktop::tao::dpi::LogicalSize};




fn main() {
    

    dioxus::desktop::launch_cfg(app, |c| {
        c.with_window(|w| {
            w.with_title("Brainfuck Interpreter")
                .with_resizable(false)
                .with_inner_size(LogicalSize::new(650, 600))
        })
    });

}



fn app(cx: Scope) -> Element {
    
    let code = use_state(&cx, || String::from("++++++++[>++++++<-]>."));

    let delay = use_state(&cx, || 2000);

    

    
    

    cx.render(rsx! {
        style { [include_str!("./style.css")] }
        div {
            id: "memorylistdiv",
            ul {
                id: "memorylist",
                /*bf.read().get_cell_list().iter().map(|value| {
                    rsx!{
                        div{
                            class: "memorydiv",
                            li{
                                class: "memory",
                                "{value}"
                            },

                            
                        }
                        
                    }
                })*/
            }

            
        }
        div{
            id: "pointerdiv",
            img{
                id: "pointer",
                src: "ok.svg",
            }
        }

        div{
            id: "codeareadiv",
            textarea{
                id: "codearea",
                value: "{code}",
                oninput: move |e| {
                    code.set(e.value.clone());
                },
            }
        }

        div{
            id: "outputdiv",
            
            input{
                id: "output",
                r#type: "text",
                disabled: "true",
               
            }
        }
        div{
            id: "controldiv",
            div{
                label{
                    "Delay:",
                }
            }
            div{
                id:"controlspeeddiv",
                input{
                    r#type: "range",
                    max: "2000",
                    value: "{delay}",
                    oninput: move |e| {
                        if let Ok(number) = e.value.parse::<u16>(){
                            
                        }
                    },
                }
                div{
                    id: "showspeeddiv",
                    input{
                        id: "showspeed",
                        r#type: "number",
                        max: "2000",
                        value: "{delay}",
                        oninput: move |e| {
                            if let Ok(mut number) = e.value.parse::<u16>(){
                                
                            }
                        },
                    }
                    label{
                        margin_left: "2px",
                        "ms",
                    }
                }
                
            }
            
            

        }
        div{
            id: "interpretediv",
            input{
                id: "interpretebutton",
                r#type: "button",
                value: "Interprete",
                onclick: move |_| {
                    
                    cx.spawn(async move{
                       
                    });
                },
            }
        }
    })
}



