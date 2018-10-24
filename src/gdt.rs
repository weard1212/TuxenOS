use x86_64::VirtAddr;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor};
use x86_64::structures::gdt::SegmentSelector;

pub const DOUBLE_FAULT_IST_INDEX: u16 =0;

pub fn init(){
    use x86_64::instructions::segmentation::set_cs;
    use x86_64::instructions::tables::load_tss;
    
    GDT.0.load();
    
    unsafe{
        set_cs(GDT.1.code_selector);
        load_tss(GDT.1.tss_selector);
    }
}

//this contains a separate double fault stack in the interrupt stack table
// the memory is defined backwards because x86 stacks grow downwards(high to low)
// memory management isn't yet implemented so it is possible to overflow this 
// stack as well but that would take a lot of errors so it probable won't happen...
lazy_static!{
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
            
            let stack_start = VirtAddr::from_ptr(unsafe{ &STACK });
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
        tss
    };
}

// this is a structure that contains the segments of the program.(thank you legacy artifacts!)
// gdt is primarily used for switching kernel space and user space, and loading TSS structure
lazy_static!{
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (gdt, Selectors { code_selector, tss_selector })
    };
}

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}
