use super::methods::{MethodError, Methods};
use super::QueryString;
use std::convert::TryFrom;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

#[derive(Debug)]
pub struct Request<'buf> {
    method: Methods,
    path: &'buf str,
    query: Option<QueryString<'buf>>,
}

// trait for Request presa da TryFrom
impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        // Questo match fanno la stessa cosa della riga 25 grazie all'implementazione di From per ParseError alla riga 31
        // match str::from_utf8(buf) {
        //     Ok(request) => {},
        //     Err(_) => return Err(ParseError::InvalidEncoding),
        // }
        let request = str::from_utf8(buf)?;
        //usando lo shadowing delle variabili, request ogni volta è il pezzo di stringa successiva
        //ecco perché nella tupla posso estrarre il valore che mi serviva e poi il resto della stringa nella request
        let (method, request) = get_next_word(request).ok_or(ParseError::InvaildRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvaildRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvaildRequest)?;
        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Methods = method.parse()?;
        let mut query_string = None;

        //questo è uguale alla riga 46 che è più pulita
        // match path.find("?") {
        //     Some(i) => {
        //         query_string = Some(&path[i + 1..]);
        //         path = &path[..i];
        //     },
        //     None => {}
        // }

        if let Some(i) = path.find("?") {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }
        Ok(Self {
            path,
            method,
            query: query_string,
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        //con enumerate mi ritorna la tupla con index e value
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }
    None
}

// CUSTOM ERRORS ENUM AND METHODS

pub enum ParseError {
    InvaildRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvaildRequest => "Invalid request",
            Self::InvalidEncoding => "Invalid encoding",
            Self::InvalidProtocol => "Invalid protocol",
            Self::InvalidMethod => "Invalid method",
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}