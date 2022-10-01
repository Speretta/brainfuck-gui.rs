mod core;


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
    let mut cell_list = [0; 32];

    let code = use_state(&cx, || String::from(r#"+++[>+++++<-]>>+<[>>++++>++>+++++>+++++>+>>+<++[++<]>---]

    >++++.>>>.+++++.>------.<--.+++++++++.>+.+.<<<<---.[>]<<.<<<.-------.>++++.
    <+++++.+.>-----.>+.<++++.>>++.>-----.
    
    <<<-----.+++++.-------.<--.<<<.>>>.<<+.>------.-..--.+++.-----<++.<--[>+<-]
    >>>>>--.--.<++++.>>-.<<<.>>>--.>.
    
    <<<<-----.>----.++++++++.----<+.+++++++++>>--.+.++<<<<.[>]<.>>
    
    ,[>>+++[<+++++++>-]<[<[-[-<]]>>[>]<-]<[<+++++>-[<+++>-[<-->-[<+++>-
    [<++++[>[->>]<[>>]<<-]>[<+++>-[<--->-[<++++>-[<+++[>[-[-[-[->>]]]]<[>>]<<-]
    >[<+>-[<->-[<++>-[<[-]>-]]]]]]]]]]]]]
    
    <[
        -[-[>+<-]>]
        <[<<<<.>+++.+.+++.-------.>---.++.<.>-.++<<<<.[>]>>>>>>>>>]
        <[[<]>++.--[>]>>>>>>>>]
        <[<<++..-->>>>>>]
        <[<<..>>>>>]
        <[<<..-.+>>>>]
        <[<<++..---.+>>>]
        <[<<<.>>.>>>>>]
        <[<<<<-----.+++++>.----.+++.+>---.<<<-.[>]>]
        <[<<<<.-----.>++++.<++.+++>----.>---.<<<.-[>]]
        <[<<<<<----.>>.<<.+++++.>>>+.++>.>>]
        <.>
    ]>
    ,]
    
    <<<<<.<+.>++++.<----.>>---.<<<-.>>>+.>.>.[<]>++.[>]<.
    >"#));

    let speed = use_state(&cx, || 2000u16);


    cx.render(rsx! {
        style { [include_str!("./style.css")] }
        div {
            id: "memorylistdiv",
            ul {
                id: "memorylist",
                (0..cell_list.len()).into_iter().map(|_|{
                    rsx!{
                        div{
                            class: "memorydiv",
                            li{
                                class: "memory",
                                "0"
                            },

                            
                        }
                        
                    }
                })
            
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
                    "Speed:",
                }
            }
            div{
                id:"controlspeeddiv",
                input{
                    r#type: "range",
                    max: "2000",
                    value: "{speed}",
                    oninput: move |e| {
                        if let Ok(number) = e.value.parse::<u16>(){
                            speed.set(number);
                        }
                    },
                }
                div{
                    id: "showspeeddiv",
                    input{
                        id: "showspeed",
                        r#type: "number",
                        max: "2000",
                        value: "{speed}",
                        oninput: move |e| {
                            if let Ok(number) = e.value.parse::<u16>(){
                                speed.set(number);
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
            }
        }
    })
}



