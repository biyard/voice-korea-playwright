#![allow(non_snake_case)]
use crate::presentations::dashboard::StatusButton;
use dioxus::prelude::*;

#[component]
pub fn DashboardCard(
    survey_type: String,
    title: String,
    update_date: String,
    response_count: u64,
    total_response_count: u64,
    draft_label: String,
    in_progress_label: String,
    complete_label: String,
    update_date_label: String,
    add_question_description: String,
    response: String,
    edit_survey: String,
    analysis_result: String,
) -> Element {
    let dashboard_card_style = "
    display: flex;
    flex-direction: column;
    width: 380px;
    height: 420px;
    border-radius: 8px;
    justify-content: space-between;
    align-items: flex-start;
    background-color: white;
    border: 1px solid #d2d2d2;
    margin: 35px;
    padding: 30px;
    ";

    rsx! {
        div {
            class: "{dashboard_card_style}",
            div {
                StatusButton {
                    survey_type: survey_type.clone(),
                    draft_label,
                    in_progress_label,
                    complete_label
                }
                div {
                    class: "w-full text-[#4c4c4c] font-semibold text-[30px] mb-[14px] overflow-hidden truncate",
                    {title}
                }
                div {
                    class: "text-[20px] font-normal text-[#4c4c4c]",
                    "{update_date_label} {update_date}"
                }
            }
            div {
                class: "flex flex-col w-full",
                div {
                    class: "flex flex-row w-full justify-start items-center mb-[37px]",
                    img {
                        class: "mr-[10px] mb-[3px]",
                        src: "/images/info.png",
                        alt: "Info"
                    }
                    if survey_type.clone() == "draft" {
                        div {
                            class: "text-[#1e5eaf] font-normal text-[20px] mt-[1px]",
                            "{add_question_description}"
                        }
                    } else {
                        div {
                            class: "text-[#1e5eaf] font-normal text-[20px] mb-[2px]",
                            "{response_count}/{total_response_count}{response}"
                        }
                    }
                }
                if survey_type == "draft" {
                    div {
                        class: "flex flex-row w-full h-[55px] rounded-[8px] border-solid border-[3px] border-[#1e5eaf] bg-white items-center justify-center",
                        div {
                            class: "text-[20px] font-medium text-[#1e5eaf]",
                            "{edit_survey}"
                        }
                    }
                } else {
                    div {
                        class: "flex flex-row w-full h-[55px] rounded-[8px] border-solid border-[3px] border-[#1e5eaf] bg-white items-center justify-center",
                        div {
                            class: "text-[20px] font-medium text-[#1e5eaf]",
                            "{analysis_result}"
                        }
                    }
                }
            }
        }
    }
}