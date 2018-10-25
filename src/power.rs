
#[no_mangle]
pub unsafe extern fn tshutdownet(){
    use x86_64::instructions::port::Port;
    let mut port = Port::<u32>::new(0xf4);
    port.write(0);
}