use radius2::{Radius, RadiusOption};

fn main() {
    let options = vec![RadiusOption::Permissions(true)];
    let mut radius = Radius::new_with_options(Some("tests/ais3"), &options);
    //let verify = radius.get_address("sym.verify").unwrap();
    let mut state = radius.call_state(0x004005f6);
    let addr: u64 = 0xfff00000;
    let flag_val = state.symbolic_value("flag", 24 * 8);
    state.memory.write_value(addr, &flag_val, 24);
    state.registers.set("rax", state.concrete_value(addr, 64));

    radius.breakpoint(0x00400602);
    radius.avoid(&[0x0040060e]);
    let new_state = radius.run(state, 1);
    //println!("{:?}", radius.processor.crashes[0].status);
    let flag = new_state.unwrap().evaluate_string(&flag_val).unwrap();
    println!("FLAG: {}", flag);
}
