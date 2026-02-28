use super::token::TokenKind;
use super::node::{Node,SyntaxErrorKind};



#[racc::grammar]
mod grammar {
    type Token = super::token::TokenKind;

    Program -> Result<Node, SyntaxErrorKind> :
        Elements(elems) { Ok(Node::ProgramNode(elems?)) };

    List -> Result<Node, SyntaxErrorKind> :
        LParent Elements(elems) RParen { Ok(Node::ListNode(elems?)) };
    
    Elements -> Result<Node, SyntaxErrorKind> :    
        Element(elem) { Ok(Node::ElementsNode(vec![elem?])) }
        | Elements(elems) Element(elem) {
            match elems? {
                Node::ElementsNode(elems_vec) => {
                    elems_vec.append(elem?);
                    Ok(Node::ElementsNode(elems_vec))
                }
                _ => {
                    Err(SyntaxErrorKind::UnexpectedParse(format!("Expected 'Elements'. Got {}", type_of(elems))))
                }
            }
        } ;

    Identifier -> Result<Node, SyntaxErrorKind> :
        Identifier(identifier) { Ok(Node::Indentifier(identifier)) };
    
    Element -> Result<Node, SyntaxErrorKind> :
        Literal(lit) { lit }
        | Identifier(ident) { ident }
        | List(list) { list };
    
    Literal -> Result<Node, SyntaxErrorKind> :
        Integer(int_str) {
             match int_str.parse::<i64>() {
                Ok(int_val) => Ok(Node::IntNode(int_val))
                _ => Err(SyntaxErrorKind::InvalidNumber(int_str))
            }
        }
        | Real(real_str) {
            match real_str.parse::<f64>() {
                Ok(real_val) => Ok(Node::RealNode(real_val))
                _ => Err(SyntaxErrorKind::InvalidNumber(real_str))
            }
        }
        | Bool(val) { Ok(Node::BoolNode(val)) }
        | Null { Ok(Node::NullNode) };
}
