use crate::utils::context::Language;

pub struct CreateTranslate {
    pub authorization: String,
    pub individual: String,
    pub company: String,
    pub individual_description: String,
    pub phone: String,
    pub phone_description: String,
    pub check_title: String,
    pub check_description_1: String,
    pub check_description_2: String,
    pub check_description_3: String,
    pub company_name: String,
    pub business_register_number: String,
    pub company_name_example: String,
    pub business_register_number_example: String,
    pub next: String,
    pub agree_terms: String,
    pub agree_membership_terms: String,
    pub agree_privacy_policy: String,
    pub entrust_personal_information: String,
    pub essential: String,
}

pub fn translate(lang: Language) -> CreateTranslate {
    match lang {
        Language::En => CreateTranslate {
            authorization: "Identity Verification".to_string(),
            individual: "Individual".to_string(),
            company: "Corporation".to_string(),
            individual_description: "This is the identity verification step for membership registration. If you are under 14 years of age, you must participate with your parents (legal representative). Please prepare your authentication method in advance.".to_string(),
            phone: "Cellphone".to_string(),
            phone_description: "Authentication by receiving a verification number sent to a mobile phone in your name".to_string(),
            check_title: "Check it out!".to_string(),
            check_description_1: "- Identity verification is only possible using a mobile phone activated in your name.".to_string(),
            check_description_2: "- If the identity verification process does not work properly, please contact XXXX (XXXX-XXXX) for mobile phone identity verification.".to_string(),
            check_description_3: "- If you have any other questions about membership registration, please contact our customer service center (0000-0000).".to_string(),
            company_name: "Company Name".to_string(),
            business_register_number: "Register Number".to_string(),
            company_name_example: "Biyard Co".to_string(),
            business_register_number_example: "000-00-00000".to_string(),
            next: "Next".to_string(),
            agree_terms: "Agree to Terms and Conditions".to_string(),
            agree_membership_terms: "Agree to membership terms and conditions".to_string(),
            agree_privacy_policy: "Privacy policy".to_string(),
            entrust_personal_information: "Entrustment of personal information processing".to_string(),
            essential: "(Essential)".to_string(),
        },
        Language::Ko => CreateTranslate {
            authorization: "본인인증".to_string(),
            individual: "개인".to_string(),
            company: "법인".to_string(),
            individual_description: "회원가입을 위한 본인확인 단계입니다. 만14세 미만인 경우 부모님(법정대리인)과 함께 진행하셔야 합니다. 인증수단을 미리 준비해주세요.".to_string(),
            phone: "휴대폰".to_string(),
            phone_description: "본인 명의로 된 휴대폰으로 인증번호를 전송 받아 인증".to_string(),
            check_title: "확인하세요!".to_string(),
            check_description_1: "- 본인인증은 본인명의로 개통된 휴대폰으로만 가능합니다.".to_string(),
            check_description_2: "- 본인인증 절차가 정상적으로 이루어지지 않을 경우 휴대폰 본인인증은 XXXX(XXXX-XXXX)로 문의하시기 바랍니다.".to_string(),
            check_description_3: "- 회원가입에 대한 다른 궁금한 사항은 고객센터(0000-0000)로 문의하여 주시기 바랍니다.".to_string(),
            company_name: "회사명".to_string(),
            business_register_number: "사업자 등록번호".to_string(),
            company_name_example: "Biyard Co".to_string(),
            business_register_number_example: "000-00-00000".to_string(),
            next: "다음".to_string(),
            agree_terms: "약관 동의".to_string(),
            agree_membership_terms: "회원약관 동의".to_string(),
            agree_privacy_policy: "개인정보처리방침".to_string(),
            entrust_personal_information: "개인정보처리의 위탁".to_string(),
            essential: "(필수)".to_string(),
        },
    }
}
