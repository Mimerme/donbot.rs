//Reads Dolphin memory and clips melee combos
//Windows only


mod mem_lib;

fn main() {
        
    let pid = mem_lib::get_proc_id_by_name("Dolphin.exe");
    println!("Found process with id {}", pid);
    
    let proccess = mem_lib::get_proc(pid);
    
    println!("Read {}", proccess.read_address("0x99B720C"));

}
