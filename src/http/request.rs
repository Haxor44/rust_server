use crate::http::request;

use super::method::Method;
use super::method::MethodError;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Debug;
use std::fmt::Result as fmtResult;
use std::str;
use super::{QueryString};

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str{
        &self.path
    }

    pub fn method(&self) -> &Method{
        &self.method
    }

    pub fn query_string(&self) -> Option<&QueryString>{
        self.query_string.as_ref()
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {

        let request = str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?;

        match get_next_word(request) {
            Some((method,request)) => {},
            None => return  Err(ParseError::InvalidRequest)
        }

        let (method,request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path,request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol,_) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        let method:Method = method.parse()?;

        let mut query_string = None;

        if let Some(i) = path.find('?')  {
            query_string = Some(QueryString::from(&path[i+1..]));
            path = &path[..i];
        }
        
        Ok(Self{
            path,
            query_string,
            method,
        })
    }
}

fn get_next_word(request:&str)-> Option<(&str,&str)>{
    for (i,c) in request.chars().enumerate()  {
        if c == ' ' || c == '\r' {
            return Some((&request[..i],&request[i+1..]));
        }
        
    }
    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl Display for ParseError{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmtResult {
        write!(f,"{}",self.message())
    }
}

impl Debug for ParseError{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmtResult {
        write!(f,"{}",self.message())
    }
}

impl ParseError {
    fn message(&self)->&str{
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}
impl Error for ParseError{

}
