extern crate x86_64;
use x86_64::structures::idt::{InterruptDescriptorTable, ExceptionStackFrame};
use gdt;
use pic8259_simple::ChainedPics;
use spin;
use hlt_loop;

//because the first 32 numbers are exception slots.
pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;
pub const TIMER_INTERRUPT_ID: u8 = PIC_1_OFFSET;
pub const KEYBOARD_INTERRUPT_ID: u8 = PIC_1_OFFSET + 1;

// mutex wrapper to provide safer memory access
pub static PICS: spin::Mutex<ChainedPics> = spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe{
            //double fault redirected to second stack
            idt.double_fault.set_handler_fn(double_fault_handler).set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        //timer interrupts
        idt[usize::from(TIMER_INTERRUPT_ID)].set_handler_fn(timer_interrupt_handler);

        //keyboard interrupts
        idt[usize::from(KEYBOARD_INTERRUPT_ID)].set_handler_fn(keyboard_interrupt_handler);

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

// this is a double fault handler. it cautches the unhandled faults and
// prevents triple faults which cause system resets and cannot be
// recovered from.
extern "x86-interrupt" fn double_fault_handler( stack_frame: &mut ExceptionStackFrame, _error_code: u64){
    println!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
    hlt_loop();
}

//
// INTERUPTS
//

//timer interrupts
extern "x86-interrupt" fn timer_interrupt_handler( stack_frame: &mut ExceptionStackFrame){
    //print!(".");

    // tells the system that we are done handling this interruption
    unsafe{ PICS.lock().notify_end_of_interrupt(TIMER_INTERRUPT_ID)}
}

// keyboard interrupts
extern "x86-interrupt" fn keyboard_interrupt_handler( stack_frame: &mut ExceptionStackFrame){

    use x86_64::instructions::port::Port;
    let port = Port::new(0x60);// the port for the keyboard
    let scancode: u8 = unsafe { port.read() };// the key that was pressed last

    // all of the keycodes for more keycodes https://wiki.osdev.org/Keyboard#Scan_Code_Set_1
    let key = match scancode {
        0x02 => Some('1'),
        0x03 => Some('2'),
        0x04 => Some('3'),
        0x05 => Some('4'),
        0x06 => Some('5'),
        0x07 => Some('6'),
        0x08 => Some('7'),
        0x09 => Some('8'),
        0x0a => Some('9'),
        0x0b => Some('0'),
        0x0C => Some('-'),
        0x0D => Some('='),
        //0x0E => Some(''), this is backspace
        0x0F => Some('\t'),
        0x10 => Some('Q'),
        0x11 => Some('W'),
        0x12 => Some('E'),
        0x13 => Some('R'),
        0x14 => Some('T'),
        0x15 => Some('Y'),
        0x16 => Some('U'),
        0x17 => Some('I'),
        0x18 => Some('O'),
        0x19 => Some('P'),
        0x1A => Some('['),
        0x1B => Some(']'),
        0x1c => Some('\n'),
        //0x1d => Some(''), left control
        0x1e => Some('A'),
        0x1f => Some('S'),
        0x20 => Some('D'),
        0x21 => Some('F'),
        0x22 => Some('G'),
        0x23 => Some('H'),
        0x24 => Some('J'),
        0x25 => Some('K'),
        0x26 => Some('L'),
        0x27 => Some(';'),
        0x28 => Some('\''),
        //0x29 => Some(''), idk
        0x2a => Some('<'), //left shift
        //0x2b => Some(''), idk
        0x2c => Some('Z'),
        0x2d => Some('X'),
        0x2e => Some('C'),
        0x2f => Some('V'),
        0x30 => Some('B'),
        0x31 => Some('N'),
        0x32 => Some('M'),
        0x33 => Some(','),
        0x34 => Some('.'),
        0x35 => Some('/'),
        0x36 => Some('>'), //Right shift
        0xB9 => Some(' '), //Space
        0x01  => Some('\r'),
        _ => None,
    };
    if let Some(key) = key {

        print!("{}", key);
    }

    unsafe{ PICS.lock().notify_end_of_interrupt(KEYBOARD_INTERRUPT_ID)}

}
