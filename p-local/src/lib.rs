mod function;

use pal::function::*;

pub fn function_runtime() -> Box<dyn FunctionRuntime> { 
    Box::new(function::FunctionRuntimeImpl)
}
