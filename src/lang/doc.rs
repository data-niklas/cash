use std::collections::HashMap;
use lazy_static::*;

lazy_static! {
    pub static ref FUNCTIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        //Math
        m.insert("abs", "Returns the absolute value of a number");
        m.insert("ceil", "Ceils the number");
        m.insert("floor", "Floors the number");
        m.insert("round", "Rounds the number to the closest int");
        m.insert("signum", "Returns 1.0 if the number is >= +0.0, else -1.0 (for <= -0.0)");
        m.insert("sin", "sin function");
        m.insert("cos", "cos function");
        m.insert("tan", "tan function");
        m.insert("asin", "asin function");
        m.insert("acos", "acos function");
        m.insert("atan", "atan function");
        m.insert("sinh", "sinh function");
        m.insert("cosh", "cosh function");
        m.insert("tanh", "tanh function");
        m.insert("asinh", "asinh function");
        m.insert("acosh", "acosh function");
        m.insert("atanh", "atanh function");
        m.insert("log", "Calculates the logarithm to a given base");
        m.insert("lg", "Calculates the logarithm to base 10");
        m.insert("ld", "Calculates the logarithm to base 2");
        m.insert("ln", "Calculates the logarithm to base e");
        m.insert("rand", "Returns a random value between 0 and 1");

        //Type
        m.insert("type", "Returns the type of the value");
        //Casting
        m.insert("int", "Attempts to cast a value to an int");
        m.insert("string", "Attempts to cast a value to an int");
        m.insert("float", "Attempts to cast a value to an int");
        m.insert("bool", "Attempts to cast a value to an int");
        
        m.insert("print", "Prints all args after each other");
        m.insert("println", "Prints all args after each other and inserts newlines");
        m.insert("vars", "Print all vars");
        m.insert("len", "Length of an array / string");

        //Control
        m.insert("quit", "Exits cash");
        m.insert("exit", "Exits cash");
        m.insert("clear", "Clears the screen");
        m.insert("cls", "Clears the screen");
        m.insert("return", "Returns from the current block and returns up to 1 result");
        m.insert("help", "shows this help or shows the help of a given command");
        m
    };
}