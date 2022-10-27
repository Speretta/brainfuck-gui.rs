pub mod streams;
mod error;


use async_recursion::async_recursion;
use dioxus::prelude::UseState;
use std::{collections::VecDeque};

use crate::core::error::BrainFuckError;

use self::streams::{BrainFuckStream, Input};


#[derive(PartialEq, Props)]
pub struct BrainFuck<const N: usize>{
    cell_list: [u8; N],
    ptr: usize,
    stream: BrainFuckStream,
}


impl<const N: usize> BrainFuck<N>{
    pub fn new(cell_list: [u8; N]) -> Self{
        BrainFuck { cell_list, ptr: 0, stream: BrainFuckStream { input: streams::Input::Wait, output_queue: VecDeque::new() }}
    }


    #[async_recursion]
    pub async fn tokenize(&self, raw_code: &'static str) -> Vec<Token>{
        fn find_bracket(code: &str) -> Result<&str, BrainFuckError>{
            let mut bracket_count = 0usize;
            for (i, ch) in code.chars().enumerate(){
                match ch{
                    '[' => {
                        if let Some(n) = bracket_count.checked_add(1){
                            bracket_count = n; 
                        }else{
                            return Err(BrainFuckError::Bracket(i));
                        }
                    }
                    ']' => {
                        if let Some(n) = bracket_count.checked_sub(1){
                            if n == 0{
                                if let Some(code_part) = code.get(1..i){
                                    return Ok(code_part);
                                }
                                
                            }
                            bracket_count = n; 
                        }else{
                            return Err(BrainFuckError::Bracket(i));
                        }
                    }
                    _ => {
                        
                    }
                }
            }
            Err(BrainFuckError::Bracket(0))
        }

        let mut token_list = Vec::new();
        for (i, ch) in raw_code.chars().enumerate(){
            match ch{
                '+' => {
                    if let Some(Token::ChangeCell(change)) = token_list.last_mut(){
                        if let Some(n) = change.checked_add(1){
                            *change = n;
                        }else{
                            token_list.push(Token::ChangeCell(1));
                        }
                    }else{
                        token_list.push(Token::ChangeCell(1));
                    }
                }
                '-' => {
                    if let Some(Token::ChangeCell(change)) = token_list.last_mut(){
                        if let Some(n) = change.checked_sub(1){
                            *change = n;
                        }else{
                            token_list.push(Token::ChangeCell(-1));
                        }
                    }else{
                        token_list.push(Token::ChangeCell(-1));
                    }
                }
                '>' => {
                    if let Some(Token::ChangePtr(change)) = token_list.last_mut(){
                        if let Some(n) = change.checked_add(1){
                            *change = n;
                        }else{
                            token_list.push(Token::ChangePtr(1));
                        }
                    }else{
                        token_list.push(Token::ChangePtr(1));
                    }
                }
                '<' => {
                    if let Some(Token::ChangePtr(change)) = token_list.last_mut(){
                        if let Some(n) = change.checked_sub(1){
                            *change = n;
                        }else{
                            token_list.push(Token::ChangePtr(-1));
                        }
                    }else{
                        token_list.push(Token::ChangePtr(-1));
                    }
                }
                '.' => {
                    token_list.push(Token::Output);
                }
                ',' => {
                    token_list.push(Token::Input);
                }
                '[' => {
                    if let Some(raw_code) = raw_code.get(i..){
                        match find_bracket(raw_code) {
                            Ok(code_part) => token_list.push(Token::Group(self.tokenize(code_part).await)),
                            Err(e) => panic!("{:?}", e),
                        }
                    }
                    
                }
                _ => {}
            }
        }        
        token_list
    }


    pub fn interprete(&mut self, token_list: &Vec<Token>){
        for token in token_list{
            match token{
                Token::ChangeCell(n) => {
                    if let Some(cell) = self.cell_list.get_mut(self.ptr){
                        *cell = cell.overflowing_add(*n as u8).0;
                    }
                },
                Token::ChangePtr(mut n) => {
                    if n.is_negative(){
                        n = n*-1;
                        self.ptr = self.ptr.overflowing_sub(n as usize).0;
                    }else{
                        self.ptr = self.ptr.overflowing_add(n as usize).0;
                    }
                    
                },
                Token::Group(tokens) => {
                    'group: loop {
                        if let Some(n) = self.cell_list.get(self.ptr){
                            if *n > 1{
                                self.interprete(tokens);
                            }else{
                                break 'group;
                            }
                        }else{
                            break 'group;
                        }
                    }
                },
                Token::Input => {
                    
                
                    loop{
                        if let Input::Ok(n) = self.stream.input{
                            if let Some(ch_number) = self.cell_list.get_mut(self.ptr){
                                *ch_number = n;
                                
                            }
                            break;
                        }
                    }
                    
                    
                   
                },
                Token::Output => {
                    if let Some(ch_number) = self.cell_list.get(self.ptr){
                        self.stream.output_queue.push_front(*ch_number);
                        
                    }
                },
            }
        }
    }

    pub fn get_stream(&self) -> &BrainFuckStream{
        &self.stream
    }
}







#[derive(Debug)]
pub enum Token{
    ChangeCell(i8),
    ChangePtr(i8),
    Group(Vec<Token>),
    Input,
    Output,
}