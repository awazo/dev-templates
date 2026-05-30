use crate::def_code_enum_str;
use crate::endpoint::code_enum::CodeEnum;

def_code_enum_str!(ExampleCode, No, {
    No = ("0", "いいえ"),
    Yes = ("1", "はい"),
});
