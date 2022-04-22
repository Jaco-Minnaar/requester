use core::fmt::{self, Display, Formatter};
use core::str::FromStr;
use diesel_derive_enum::DbEnum;

#[derive(Debug, Copy, Clone, PartialEq, DbEnum)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

impl Display for HttpMethod {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let repr = match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Patch => "PATCH",
            HttpMethod::Delete => "DELETE",
        };
        write!(f, "{}", repr)
    }
}

type HttpMethodErr = String;

impl FromStr for HttpMethod {
    type Err = HttpMethodErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(HttpMethod::Get),
            "POST" => Ok(HttpMethod::Post),
            "PUT" => Ok(HttpMethod::Put),
            "PATCH" => Ok(HttpMethod::Patch),
            "DELETE" => Ok(HttpMethod::Delete),
            x => Err(format!("Unrecognized HTTP method: {}", x).into()),
        }
    }
}

// impl<DB> FromSql<Text, DB> for HttpMethod where DB: Backend, String: FromSql<Text, DB> {
//     fn from_sql(bytes: Option<&DB::RawValue>) -> diesel::deserialize::Result<Self> {
//         match String::from_sql(bytes)?.as_str() {
//             "GET" => Ok(HttpMethod::Get),
//             "POST" => Ok(HttpMethod::Post),
//             "PUT" => Ok(HttpMethod::Put),
//             "PATCH" => Ok(HttpMethod::Patch),
//             "DELETE" => Ok(HttpMethod::Delete),
//             x => Err(format!("Unrecognized HTTP method: {}", x).into())
//         }
//     }
// }
