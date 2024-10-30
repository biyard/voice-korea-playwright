#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::{
    components::{input::Input, table_row::Row},
    prelude::*,
};

#[derive(PartialEq, Props, Clone)]
pub struct ResetPasswordPageProps {
    lang: Language,
}

#[derive(PartialEq, Props, Clone)]
pub struct SendAuthenticationButtonProps {
    width: Option<i64>,
    label: String,
    lang: Language,
    onclick: EventHandler<MouseEvent>,
}

#[derive(PartialEq, Props, Clone)]
pub struct EmailAuthenticationi18nProps {
    reset_password: String,
    email_address_label: String,
    name_label: String,
    name_hint: String,
    phone_label: String,
    phone_hint: String,
    send_authentication: String,
    authentication_number_label: String,
    authentication_number_description: Vec<String>,
    check_title: String,
    check_description: Vec<String>,
}

#[derive(PartialEq, Props, Clone)]
pub struct ResetNewPasswordi18nProps {
    reset_password: String,
    input_new_password_label: String,
    input_new_password_check_label: String,
    check_title: String,
    check_new_password_description: Vec<String>,
}

#[derive(PartialEq, Props, Clone)]
pub struct EmailAuthenticationProps {
    ctrl: controller::Controller,
    lang: Language,
    i18n: EmailAuthenticationi18nProps,
}

#[derive(PartialEq, Props, Clone)]
pub struct ResetNewPasswordProps {
    ctrl: controller::Controller,
    lang: Language,
    i18n: ResetNewPasswordi18nProps,
}

#[derive(PartialEq, Props, Clone)]
pub struct CompleteResetPasswordProps {
    ctrl: controller::Controller,
    lang: Language,
    i18n: CompleteResetPasswordi18nProps,
}

#[derive(PartialEq, Props, Clone)]
pub struct CompleteResetPasswordi18nProps {
    complete_change_password_title: String,
    complete_change_password_description: Vec<String>,
    go_to_login: String,
}

pub mod controller;
pub mod i18n;

#[component]
pub fn ResetPasswordPage(props: ResetPasswordPageProps) -> Element {
    let mut ctrl = controller::Controller::init();
    let translates = i18n::translate(props.lang.clone());

    rsx! {
        div {
            class: "flex flex-col w-full h-full items-start justify-center px-[170px] py-[85px]",
            if ctrl.get_step() == 0 {
                EmailAuthentication {
                    ctrl,
                    lang: props.lang,
                    i18n: EmailAuthenticationi18nProps {
                        reset_password: translates.reset_password,
                        email_address_label: translates.email_address_label,
                        name_label: translates.name_label,
                        name_hint: translates.name_hint,
                        phone_label: translates.phone_label,
                        phone_hint: translates.phone_hint,
                        send_authentication: translates.send_authentication,
                        authentication_number_label: translates.authentication_number_label,
                        authentication_number_description: translates.authentication_number_description,
                        check_title: translates.check_title,
                        check_description: translates.check_description,
                    }
                }
            }
            else if ctrl.get_step() == 1 {
                ResetNewPassword {
                    ctrl,
                    lang: props.lang,
                    i18n: ResetNewPasswordi18nProps {
                        reset_password: translates.reset_password,
                        input_new_password_label: translates.input_new_password_label,
                        input_new_password_check_label: translates.input_new_password_check_label,
                        check_title: translates.check_title,
                        check_new_password_description: translates.check_new_password_description
                    }
                }
            } else {
                CompleteResetPassword {
                    ctrl,
                    lang: props.lang,
                    i18n: CompleteResetPasswordi18nProps {
                        complete_change_password_title: translates.complete_change_password_title,
                        complete_change_password_description: translates.complete_change_password_description,
                        go_to_login: translates.go_to_login,
                    }
                }
            }
        }
    }
}

#[component]
pub fn CompleteResetPassword(props: CompleteResetPasswordProps) -> Element {
    rsx! {
        div {
            class: "flex flex-col w-full h-full justify-center items-center pt-[160px]",
            img {
                class: "flex flex-col pb-[30px]",
                src: "/images/check_mark.png",
                alt: "Checkmark"
            }
            div {
                class: "text-[26px] font-bold text-black pb-[30px]",
                "{props.i18n.complete_change_password_title}"
            }
            div {
                class: "text-[20px] font-normal text-black",
                "{props.i18n.complete_change_password_description[0]}"
            }
            div {
                class: "text-[20px] font-normal text-black mb-[110px]",
                "{props.i18n.complete_change_password_description[1]}"
            }
            div {
                class: "flex flex-row w-full justify-end items-end",
                Link {
                    to: Route::LoginPage { lang: props.lang },
                    div {
                        class: "flex flex-row w-[300px] h-[60px] bg-[#2168c3] justify-center items-center text-white font-bold text-[24px]",
                        "{props.i18n.go_to_login}"
                    }
                }
            }
        }
    }
}

#[component]
pub fn ResetNewPassword(props: ResetNewPasswordProps) -> Element {
    let mut ctrl = props.ctrl;
    rsx! {
        div {
            class: "flex flex-col w-full h-full",
            div {
                class: "text-[32px] font-bold text-black pb-[30px]",
                "{props.i18n.reset_password}"
            }
            div {
                class: "flex flex-col w-full h-full",
                div {
                    class: "flex flex-col w-full h-full items-start justify-center pb-[30px]",
                    Row {
                        enable_bottom_border: false,
                        label: props.i18n.input_new_password_label,
                        element: rsx! {
                            div {
                                class: "flex flex-row w-full h-full justify-start items-center",
                                div {
                                    class: "mx-[10px]",
                                    Input {
                                        value: ctrl.get_new_password(),
                                        input_type: "password",
                                        onchange: move |e| {
                                            ctrl.set_new_password(e);
                                        }
                                    }
                                },
                            }
                        }
                    }
                    Row {
                        enable_bottom_border: true,
                        label: props.i18n.input_new_password_check_label,
                        element: rsx! {
                            div {
                                class: "flex flex-row w-full h-full justify-start items-center",
                                div {
                                    class: "mx-[10px]",
                                    Input {
                                        value: ctrl.get_new_password_check(),
                                        input_type: "password",
                                        onchange: move |e| {
                                            ctrl.set_new_password_check(e);
                                        }
                                    }
                                },
                            }
                        }
                    }
                }
            }
            div {
                class: "flex flex-col w-full h-min min-w-[710px] border-solid border border-[#e0e0e0] px-[20px] py-[30px] mt-[30px] mb-[175px]",
                div {
                    class: "text-black text-[20px] font-normal pb-[20px]",
                    "{props.i18n.check_title}"
                }
                div {
                    class: "text-[#636363] text-[20px] font-normal pb-[5px]",
                    "{props.i18n.check_new_password_description[0]}"
                }
            }
            div {
                class: "flex flex-row w-full justify-end items-end",
                div {
                    class: "flex flex-row w-[300px] h-[60px] bg-[#2168c3] justify-center items-center text-white font-bold text-[24px]",
                    onclick: move |_| {
                        ctrl.clicked_reset_new_password();
                    },
                    "{props.i18n.reset_password}"
                }
            }
        }
    }
}

#[component]
pub fn EmailAuthentication(props: EmailAuthenticationProps) -> Element {
    let mut ctrl = props.ctrl;
    rsx! {
        div {
            class: "flex flex-col w-full h-full",
            div {
                class: "text-[32px] font-bold text-black pb-[30px]",
                "{props.i18n.reset_password}"
            }
            div {
                class: "flex flex-col w-full h-full",
                div {
                    class: "flex flex-col w-full h-full items-start justify-center pb-[30px]",
                    Row {
                        enable_bottom_border: false,
                        label: props.i18n.email_address_label,
                        element: rsx! {
                            div {
                                class: "flex flex-row w-full h-full justify-start items-center",
                                div {
                                    class: "mx-[10px]",
                                    Input {
                                        value: ctrl.get_email(),
                                        onchange: move |e| {
                                            ctrl.set_email(e);
                                        }
                                    }
                                },
                                SendAuthenticationButton {
                                    label: props.i18n.send_authentication,
                                    lang: props.lang,
                                    onclick: move |_| {
                                        // ctrl.set_click_send_authentication();
                                    }
                                }
                            }
                        }
                    }
                    // Row {
                    //     enable_bottom_border: false,
                    //     label: props.i18n.name_label,
                    //     element: rsx! {
                    //         div {
                    //             class: "flex flex-row w-full h-full justify-start items-center",
                    //             div {
                    //                 class: "mx-[10px]",
                    //                 Input {
                    //                     value: ctrl.get_name(),
                    //                     placeholder: props.i18n.name_hint,
                    //                     onchange: move |e| {
                    //                         ctrl.set_name(e);
                    //                     }
                    //                 }
                    //             },
                    //         }
                    //     }
                    // }
                    // Row {
                    //     enable_bottom_border: false,
                    //     label: props.i18n.phone_label,
                    //     element: rsx! {
                    //         div {
                    //             class: "flex flex-row w-full h-full justify-start items-center",
                    //             div {
                    //                 class: "mx-[10px]",
                    //                 Input {
                    //                     value: ctrl.get_phone_number(),
                    //                     placeholder: props.i18n.phone_hint,
                    //                     onchange: move |e| {
                    //                         ctrl.set_phone_number(e);
                    //                     }
                    //                 }
                    //             },
                    //             SendAuthenticationButton {
                    //                 label: props.i18n.send_authentication,
                    //                 lang: props.lang,
                    //                 onclick: move |_| {
                    //                     // ctrl.set_click_send_authentication();
                    //                 }
                    //             }
                    //         }
                    //     }
                    // }
                    Row {
                        enable_bottom_border: true,
                        height: Some(135),
                        label: props.i18n.authentication_number_label,
                        element: rsx! {
                            div {
                                class: "flex flex-col w-full h-full justify-start items-start mt-[10px] ml-[10px]",
                                div {
                                    class: "pb-[5px]",
                                    Input {
                                        value: ctrl.get_authentication_number(),
                                        onchange: move |e| {
                                            ctrl.set_authentication_number(e);
                                        }
                                    }
                                },
                                div {
                                    class: "text-[16px] font-normal text-[#636363]",
                                    "{props.i18n.authentication_number_description[0]}"
                                }
                                div {
                                    class: "text-[16px] font-normal text-[#636363]",
                                    "{props.i18n.authentication_number_description[1]}"
                                }
                            }
                        }
                    }
                }
            }
            div {
                class: "flex flex-col w-full h-min min-w-[710px] border-solid border border-[#e0e0e0] px-[20px] py-[30px] my-[30px]",
                div {
                    class: "text-black text-[20px] font-normal pb-[20px]",
                    "{props.i18n.check_title}"
                }
                div {
                    class: "text-[#636363] text-[20px] font-normal pb-[5px]",
                    "{props.i18n.check_description[0]}"
                }
                div {
                    class: "text-[#636363] text-[20px] font-normal pb-[5px]",
                    "{props.i18n.check_description[1]}"
                }
                div {
                    class: "text-[#636363] text-[20px] font-normal pb-[5px]",
                    "{props.i18n.check_description[2]}"
                }
            }
            div {
                class: "flex flex-row w-full justify-end items-end",
                div {
                    class: "flex flex-row w-[300px] h-[60px] bg-[#2168c3] justify-center items-center text-white font-bold text-[24px]",
                    onclick: move |_| {
                        ctrl.clicked_email_authentication();
                    },
                    "{props.i18n.reset_password}"
                }
            }
        }
    }
}

#[component]
pub fn SendAuthenticationButton(props: SendAuthenticationButtonProps) -> Element {
    let width = match props.width {
        Some(w) => format!("w-[{}px]", w),
        None => {
            if props.lang == Language::En {
                "w-[270px]".to_string()
            } else {
                "w-[135px]".to_string()
            }
        }
    };

    rsx! {
        div {
            class: format!("flex flex-col {} h-[35px] justify-start items-start", width),
            onclick: move |evt| { props.onclick.call(evt); },
            style: "border: 1px solid; border-color: rgba(33, 104, 195, 0.5); border-radius: 5px; background-clip: padding-box; background-color: rgba(33, 104, 195, 0.04);",
            div {
                class: "flex flex-row justify-center items-center w-full h-full text-[#2168c3] font-normal text-[15px]",
                {props.label}
            }
        }
    }
}
