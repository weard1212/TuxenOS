extern crate x86_64;
use x86_64::structures::idt::{InterruptDescriptorTable, ExceptionStackFrame};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

pub fn init_idt(){
    IDT.load();
}


// breakpoint exception handler. This is how debuggers work by throughing 
// a breakpoint exception then replacing the exception call with the original 
// stack instruction. x86-interrupt is the type of code that is run
extern "x86-interrupt" fn breakpoint_handler( stack_frame: &mut ExceptionStackFrame ){
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}
