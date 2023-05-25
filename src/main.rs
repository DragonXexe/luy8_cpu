use luy8_cpu::{cpu::{CPU, memory::{DataBus, RAM}}, RAM_BYTES, load_bin_file, assambly::{assamble, dissassamble}, store_bin_file, utils::Enumerate};




fn main() {
    println!("Hello, world!");
    // _test_assamble();
    _test_dissassable();
    // _test_cpu();
}
fn _test_cpu() {
    let mut cpu = CPU::new();
    let mut data_bus = DataBus::new();
    data_bus.add_driver(Box::new(RAM::new(RAM_BYTES * 8)));
    println!("[{}:{}]", file!(), line!());
    cpu.mount_data_bus(data_bus);
    let data = load_bin_file("./data/multiply_test.bin".to_string());
    if data.is_err() {
        panic!("{:?}", data)
    }
    let data = data.unwrap();
    for i in 0..data.len() {
        let byte = data[i];
        cpu.data_bus.write_byte(i, byte);
        
    }
    for _ in 0..6 {
        cpu.clock();
        dbg!(&cpu.registers);
    }
}
fn _test_assamble() {
    let byte_code = assamble(
        r#"
    mov ax, 4
    push ax
    mov ax, 8
    push ax
    mov ax, [510]
    "#
        .to_string(),
    );
    if let Ok(byte_code) = byte_code {
        let res = store_bin_file("./data/multiply_test.bin".to_string(), byte_code);
        if res.is_ok() {
            println!("succes!!!");
        } else {
            println!("something went wrong writing to the file");
        }
    } else {
        let error = byte_code.unwrap_err();
        println!("assambling error:\n{}",error);
    }
}
fn _test_dissassable() {
    let data = load_bin_file("./data/multiply_test.bin".to_string());
    if data.is_err() {
        panic!("{:?}", data)
    }
    let data = data.unwrap();
    let assambly = dissassamble(data);
    println!("{:#?}", assambly);
    for (i, line) in assambly.unwrap().0.enumerate() {
        if line.2.is_none() {
            println!("[{}] {}", i, line.1);
        } else if line.3.is_none() {
            println!("[{}] {} {}", i, line.1, line.2.clone().unwrap());
        } else {
            println!("[{}] {} {}, {}", i, line.1, line.2.clone().unwrap(), line.3.clone().unwrap());
        }
    }
}