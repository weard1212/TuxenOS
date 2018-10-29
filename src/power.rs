#[macro_use]
use serial;

#[no_mangle]
pub unsafe extern fn shutdown(){
    serial::print(format_args!("Shutting Down"));
    use x86_64::instructions::port::Port;
    let mut port = Port::<u32>::new(0xf4);
    port.write(0);

}