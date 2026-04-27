pub const FORMATTING_ERR_MSG:       &str = "Couldn't format time data:";
const PARSING_ERR_MSG:          &str = "Couldn't parse into time data:";
const SUBMIT_PARSING_FMT_MSG:   &str = "\n\tEnsure the given time is in an accepted format. \
    \n\tIf this input should be accepted as a new format, submit an issue.";

pub const TABLE_FORMATTING_ERR_MSG: &str = "Couldn't format table of cycle times.";

pub fn parsing_context_msg(given_time: &str) -> String {
    format!("{PARSING_ERR_MSG} `{given_time}`. {SUBMIT_PARSING_FMT_MSG}")
}

pub fn formatting_context_msg(given_time: &time::Time) -> String {
    format!("{FORMATTING_ERR_MSG} `{given_time}`")
}