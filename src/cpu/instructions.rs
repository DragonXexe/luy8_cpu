use std::fmt::Debug;

use crate::cpu::CPU;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArgumentType {
    None = 0,
    Value = 1,
    Register = 2,
    Addres = 3,
    RegAddr = 4,
}
#[derive(Clone)]
pub struct Instruction {
    pub arguments: (ArgumentType, ArgumentType),
    pub handler: fn(&mut CPU, usize, usize),
}
impl PartialEq for Instruction {
    fn eq(&self, other: &Self) -> bool {
        self.arguments == other.arguments
            && self.handler as *const usize == other.handler as *const usize
    }
}
impl Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Instruction")
            .field("arguments", &self.arguments)
            .finish()
    }
}
use crate::cpu::*;
use crate::instruction_set;
instruction_set!(
// nop
instruction!(NOP, None, None, |_cpu, _, _| {})
// add
instruction!(ADDRV, Register, Value, |cpu, reg, val| {
    let a = cpu.registers.read_reg(reg.into());
    let b = val;
    let res = (a as u8).wrapping_add(b as u8) as usize;
    cpu.registers.set_carry(res < a);
    cpu.registers.set_zero(res == 0);

    cpu.registers.write_reg(
        reg.into(),
        res
    );
})
instruction!(ADDRR, Register, Register, |cpu, reg1, reg2| {
    let a = cpu.registers.read_reg(reg1.into());
    let b = cpu.registers.read_reg(reg2.into());
    let res = (a as u8).wrapping_add(b as u8) as usize;
    cpu.registers.set_carry(res < a);
    cpu.registers.set_zero(res == 0);

    cpu.registers.write_reg(
        reg1.into(),
        res
    );
})
instruction!(ADDRA, Register, Addres, |cpu, reg, addr| {
    let a = cpu.registers.read_reg(reg.into());
    let b = cpu.data_bus.read_byte(addr) as usize;
    let res = (a as u8).wrapping_add(b as u8) as usize;
    cpu.registers.set_carry(res < a);
    cpu.registers.set_zero(res == 0);

    cpu.registers.write_reg(
        reg.into(),
        res
    );
})
instruction!(ADDRRA, Register, RegAddr, |cpu, reg, reg_addr| {
    let a = cpu.registers.read_reg(reg.into());
    let b = cpu.data_bus.read_byte(reg_addr) as usize;
    let res = (a as u8).wrapping_add(b as u8) as usize;
    cpu.registers.set_carry(res < a);
    cpu.registers.set_zero(res == 0);

    cpu.registers.write_reg(
        reg.into(),
        res
    );
})
// sub
instruction!(SUBRV, Register, Value, |cpu, reg, val| {
    let a = cpu.registers.read_reg(reg.into());
    let b = val;
    let res = (a as u8).wrapping_sub(b as u8) as usize;
    cpu.registers.set_carry(res > a);
    cpu.registers.set_zero(res == 0);

    cpu.registers.write_reg(
        reg.into(),
        res
    );
})
instruction!(SUBRR, Register, Register, |cpu, reg1, reg2| {
    let a = cpu.registers.read_reg(reg1.into());
    let b = cpu.registers.read_reg(reg2.into());
    let res = (a as u8).wrapping_sub(b as u8) as usize;
    cpu.registers.set_carry(res > a);
    cpu.registers.set_zero(res == 0);

    cpu.registers.write_reg(
        reg1.into(),
        res
    );
})
instruction!(SUBRA, Register, Addres, |cpu, reg, addr| {
    let a = cpu.registers.read_reg(reg.into());
    let b = cpu.data_bus.read_byte(addr) as usize;
    let res = (a as u8).wrapping_sub(b as u8) as usize;
    cpu.registers.set_carry(res > a);
    cpu.registers.set_zero(res == 0);

    cpu.registers.write_reg(
        reg.into(),
        res
    );
})
instruction!(SUBRRA, Register, RegAddr, |cpu, reg, reg_addr| {
    let a = cpu.registers.read_reg(reg.into());
    let b = cpu.data_bus.read_byte(reg_addr) as usize;
    let res = (a as u8).wrapping_sub(b as u8) as usize;
    cpu.registers.set_carry(res > a);
    cpu.registers.set_zero(res == 0);

    cpu.registers.write_reg(
        reg.into(),
        res
    );
})
// and
instruction!(ANDRV, Register, Value, |cpu, reg, val| {
    cpu.registers
        .write_reg(reg.into(), cpu.registers.read_reg(reg.into()) & (val));
})
instruction!(ANDRR, Register, Register, |cpu, reg1, reg2| {
    cpu.registers.write_reg(
        reg1.into(),
        cpu.registers.read_reg(reg1.into()) & (cpu.registers.read_reg(reg2.into()) as usize),
    );
})
instruction!(ANDRA, Register, Addres, |cpu, reg, addr| {
    cpu.registers.write_reg(
        reg.into(),
        cpu.registers.read_reg(reg.into()) & (cpu.data_bus.read_byte(addr) as usize),
    );
})
instruction!(ANDRRA, Register, RegAddr, |cpu, reg, reg_addr| {
    cpu.registers.write_reg(
        reg.into(),
        cpu.registers.read_reg(reg.into()) & (cpu.data_bus.read_byte(reg_addr) as usize),
    );
})
// or
instruction!(ORRV, Register, Value, |cpu, reg, val| {
    cpu.registers
        .write_reg(reg.into(), cpu.registers.read_reg(reg.into()) | (val));
})
instruction!(ORRR, Register, Register, |cpu, reg1, reg2| {
    cpu.registers.write_reg(
        reg1.into(),
        cpu.registers.read_reg(reg1.into()) | (cpu.registers.read_reg(reg2.into()) as usize),
    );
})
instruction!(ORRA, Register, Addres, |cpu, reg, addr| {
    cpu.registers.write_reg(
        reg.into(),
        cpu.registers.read_reg(reg.into()) | (cpu.data_bus.read_byte(addr) as usize),
    );
})
instruction!(ORRRA, Register, RegAddr, |cpu, reg, reg_addr| {
    cpu.registers.write_reg(
        reg.into(),
        cpu.registers.read_reg(reg.into()) | (cpu.data_bus.read_byte(reg_addr) as usize),
    );
})
// xor
instruction!(XORRV, Register, Value, |cpu, reg, val| {
    cpu.registers
        .write_reg(reg.into(), cpu.registers.read_reg(reg.into()) ^ (val));
})
instruction!(XORRR, Register, Register, |cpu, reg1, reg2| {
    cpu.registers.write_reg(
        reg1.into(),
        cpu.registers.read_reg(reg1.into()) ^ (cpu.registers.read_reg(reg2.into()) as usize),
    );
})
instruction!(XORRA, Register, Addres, |cpu, reg, addr| {
    cpu.registers.write_reg(
        reg.into(),
        cpu.registers.read_reg(reg.into()) ^ (cpu.data_bus.read_byte(addr) as usize),
    );
})
instruction!(XORRRA, Register, RegAddr, |cpu, reg, reg_addr| {
    cpu.registers.write_reg(
        reg.into(),
        cpu.registers.read_reg(reg.into()) ^ (cpu.data_bus.read_byte(reg_addr) as usize),
    );
})
// not
instruction!(NOTR, Register, None, |cpu, reg, _| {
    cpu.registers
        .write_reg(reg.into(), !cpu.registers.read_reg(reg.into()));
})
// mov
instruction!(MOVRV, Register, Value, |cpu, reg, val| {
    cpu.registers.write_reg(reg.into(), val);
})
instruction!(MOVRR, Register, Register, |cpu, reg1, reg2| {
    cpu.registers
        .write_reg(reg1.into(), cpu.registers.read_reg(reg2.into()));
})
instruction!(MOVRA, Register, Addres, |cpu, reg, addr| {
    cpu.registers
        .write_reg(reg.into(), cpu.data_bus.read_byte(addr) as usize);
})
instruction!(MOVRRA, Register, RegAddr, |cpu, reg, reg_addr| {
    cpu.registers
        .write_reg(reg.into(), cpu.data_bus.read_byte(reg_addr) as usize);
})
// str
instruction!(STRRV, Register, Value, |cpu, reg, val| {
    cpu.data_bus
        .write_byte(val, cpu.registers.read_reg(reg.into()) as u8);
})
instruction!(STRRR, Register, Register, |cpu, reg1, reg2| {
    cpu.data_bus.write_byte(
        cpu.registers.read_reg(reg1.into()),
        cpu.registers.read_reg(reg2.into()) as u8,
    );
})
instruction!(STRRA, Register, Addres, |cpu, reg, addr| {
    cpu.data_bus.write_byte(
        cpu.data_bus.read_byte(addr) as usize,
        cpu.registers.read_reg(reg.into()) as u8,
    );
})
instruction!(STRRRA, Register, RegAddr, |cpu, reg, reg_addr| {
    cpu.data_bus.write_byte(
        cpu.data_bus.read_byte(reg_addr) as usize,
        cpu.registers.read_reg(reg.into()) as u8,
    );
})
// pop
instruction!(POPR, Register, None, |cpu, reg, _| {
    let stp = cpu.registers.read_reg(Register::STP);
    // dbg!(cpu.data_bus.read_byte(stp));
    cpu.registers
        .write_reg(reg.into(), cpu.data_bus.read_byte(stp) as usize);
    cpu.registers.write_reg(Register::STP, stp.wrapping_add(1));
})
// push
instruction!(PUSHV, Value, None, |cpu, val, _| {
    let stp = (cpu.registers.read_reg(Register::STP) as u16).wrapping_sub(1) as usize;
    cpu.registers.write_reg(Register::STP, stp); // decrement stp
    cpu.data_bus.write_byte(stp, val as u8); // write at stp
})
instruction!(PUSHR, Register, None, |cpu, reg, _| {
    let stp = (cpu.registers.read_reg(Register::STP) as u16).wrapping_sub(1) as usize;
    cpu.registers.write_reg(Register::STP, stp);
    cpu.data_bus.write_byte(
        stp,
        cpu.registers.read_reg(reg.into()) as u8,
    );
})
instruction!(PUSHA, Addres, None, |cpu, addr, _| {
    let stp = (cpu.registers.read_reg(Register::STP) as u16).wrapping_sub(1) as usize;
    cpu.registers.write_reg(Register::STP, stp);
    cpu.data_bus
        .write_byte(stp, cpu.data_bus.read_byte(addr));
})
instruction!(PUSHRA, RegAddr, None, |cpu, reg_addr, _| {
    let stp = (cpu.registers.read_reg(Register::STP) as u16).wrapping_sub(1) as usize;
    cpu.registers.write_reg(Register::STP, stp);
    cpu.data_bus
        .write_byte(stp, cpu.data_bus.read_byte(reg_addr));

})
// cmp
instruction!(CMPRV, Register, Value, |cpu, reg, val| {
    let a = cpu.registers.read_reg(reg.into());
    let b = val;
    cpu.registers.set_eq(a==b);
    cpu.registers.set_mr(a>b);
    cpu.registers.set_ls(a<b);
})
instruction!(CMPRR, Register, Register, |cpu, reg1, reg2| {
    let a = cpu.registers.read_reg(reg1.into());
    let b = cpu.registers.read_reg(reg2.into());
    cpu.registers.set_eq(a==b);
    cpu.registers.set_mr(a>b);
    cpu.registers.set_ls(a<b);
})
instruction!(CMPRA, Register, Addres, |cpu, reg, addr| {
    let a = cpu.registers.read_reg(reg.into());
    let b = cpu.data_bus.read_byte(addr) as usize;
    cpu.registers.set_eq(a==b);
    cpu.registers.set_mr(a>b);
    cpu.registers.set_ls(a<b);
})
instruction!(CMPRRA, Register, RegAddr, |cpu, reg, reg_addr| {
    let a = cpu.registers.read_reg(reg.into());
    let b = cpu.data_bus.read_byte(reg_addr) as usize;
    cpu.registers.set_eq(a==b);
    cpu.registers.set_mr(a>b);
    cpu.registers.set_ls(a<b);
})
// jmp
instruction!(JMPV, Value, None, |cpu, val, _| {
    cpu.registers.write_reg(Register::PC, val);
})
instruction!(JMPR, Register, None, |cpu, reg, _| {
    cpu.registers.write_reg(Register::PC, cpu.registers.read_reg(reg.into()));
})
instruction!(JMPA, Addres, None, |cpu, addr, _| {
    cpu.registers.write_reg(Register::PC, addr);
})
instruction!(JMPRA, RegAddr, None, |cpu, reg_addr, _| {
    cpu.registers.write_reg(Register::PC, reg_addr);
})
// je
instruction!(JEV, Value, None, |cpu, val, _| {
    if cpu.registers.get_eq() {
        cpu.registers.write_reg(Register::PC, val);
    }
})
instruction!(JER, Register, None, |cpu, reg, _| {
    if cpu.registers.get_eq() {
        cpu.registers.write_reg(Register::PC, cpu.registers.read_reg(reg.into()));
    }
})
instruction!(JEA, Addres, None, |cpu, addr, _| {
    if cpu.registers.get_eq() {
        cpu.registers.write_reg(Register::PC, addr);
    }
})
instruction!(JERA, RegAddr, None, |cpu, reg_addr, _| {
    if cpu.registers.get_eq() {
        cpu.registers.write_reg(Register::PC, reg_addr);
    }
})
// jz
instruction!(JZV, Value, None, |cpu, val, _| {
    if cpu.registers.get_zero() {
        cpu.registers.write_reg(Register::PC, val);
    }
})
instruction!(JZR, Register,  None, |cpu, reg, _| {
    if cpu.registers.get_zero() {
        cpu.registers.write_reg(Register::PC, cpu.registers.read_reg(reg.into()));
    }
})
instruction!(JZA, Addres,  None, |cpu, addr, _| {
    if cpu.registers.get_zero() {
        cpu.registers.write_reg(Register::PC, addr);
    }
})
instruction!(JZRA, RegAddr,  None, |cpu, reg_addr, _| {
    if cpu.registers.get_zero() {
        cpu.registers.write_reg(Register::PC, reg_addr);
    }
})
// jne
instruction!(JNEV, Value, None, |cpu, val, _| {
    if !cpu.registers.get_eq() {
        cpu.registers.write_reg(Register::PC, val);
    }
})
instruction!(JNER, Register,  None, |cpu, reg, _| {
    if !cpu.registers.get_eq() {
        cpu.registers.write_reg(Register::PC, cpu.registers.read_reg(reg.into()));
    }
})
instruction!(JNEA, Addres,  None, |cpu, addr, _| {
    if !cpu.registers.get_eq() {
        cpu.registers.write_reg(Register::PC, addr);
    }
})
instruction!(JNERA, RegAddr,  None, |cpu, reg_addr, _| {
    if !cpu.registers.get_eq() {
        cpu.registers.write_reg(Register::PC, reg_addr);
    }
})
// jmr
instruction!(JMRV, Value, None, |cpu, val, _| {
    if cpu.registers.get_mr() {
        cpu.registers.write_reg(Register::PC, val);
    }
})
instruction!(JMRR, Register,  None, |cpu, reg, _| {
    if cpu.registers.get_mr() {
        cpu.registers.write_reg(Register::PC, cpu.registers.read_reg(reg.into()));
    }
})
instruction!(JMRA, Addres,  None, |cpu, addr, _| {
    if cpu.registers.get_mr() {
        cpu.registers.write_reg(Register::PC, addr);
    }
})
instruction!(JMRRA, RegAddr,  None, |cpu, reg_addr, _| {
    if cpu.registers.get_mr() {
        cpu.registers.write_reg(Register::PC, reg_addr);
    }
})
// jls
instruction!(JLSV, Value, None, |cpu, val, _| {
    if cpu.registers.get_ls() {
        cpu.registers.write_reg(Register::PC, val);
    }
})
instruction!(JLSR, Register,  None, |cpu, reg, _| {
    if cpu.registers.get_ls() {
        cpu.registers.write_reg(Register::PC, cpu.registers.read_reg(reg.into()));
    }
})
instruction!(JLSA, Addres,  None, |cpu, addr, _| {
    if cpu.registers.get_ls() {
        cpu.registers.write_reg(Register::PC, addr);
    }
})
instruction!(JLSRA, RegAddr,  None, |cpu, reg_addr, _| {
    if cpu.registers.get_ls() {
        cpu.registers.write_reg(Register::PC, reg_addr);
    }
})
// interrupts !!! todo!!!
instruction!(INTV, Value,  None, |_cpu, _val, _| {
    todo!()
})
instruction!(INTR, Register,  None, |_cpu, _reg, _| {
    todo!()
})
instruction!(INTA, Addres,  None, |_cpu, _addr, _| {
    todo!()
})
instruction!(INTRA, RegAddr,  None, |_cpu, _reg_addr, _| {
    todo!()
})
// call
instruction!(CALLV, Value, None, |cpu, val, _| {
    // push pc
    let stp = (cpu.registers.read_reg(Register::STP) as u16).wrapping_sub(1) as usize;
    cpu.registers.write_reg(Register::STP, stp);
    cpu.data_bus.write_byte(
        stp,
        cpu.registers.read_reg(Register::PC) as u8,
    );
    // jmp reg
    cpu.registers.write_reg(Register::PC, val);
})
instruction!(CALLR, Register,  None, |cpu, reg, _| {
    // push pc
    let stp = (cpu.registers.read_reg(Register::STP) as u16).wrapping_sub(1) as usize;
    cpu.registers.write_reg(Register::STP, stp);
    cpu.data_bus.write_byte(
        stp,
        cpu.registers.read_reg(Register::PC) as u8,
    );
    // jmp reg
    cpu.registers.write_reg(Register::PC, cpu.registers.read_reg(reg.into()));
})
instruction!(CALLA, Addres,  None, |cpu, addr, _| {
    // push pc
    let stp = (cpu.registers.read_reg(Register::STP) as u16).wrapping_sub(1) as usize;
    cpu.registers.write_reg(Register::STP, stp);
    cpu.data_bus.write_byte(
        stp,
        cpu.registers.read_reg(Register::PC) as u8,
    );
    // jmp addr
    cpu.registers.write_reg(Register::PC, addr);
})
instruction!(CALLRA, RegAddr,  None, |cpu, reg_addr, _| {
    // push pc
    let stp = (cpu.registers.read_reg(Register::STP) as u16).wrapping_sub(1) as usize;
    cpu.registers.write_reg(Register::STP, stp);
    cpu.data_bus.write_byte(
        stp,
        cpu.registers.read_reg(Register::PC) as u8,
    );
    // jmp reg_addr
    cpu.registers.write_reg(Register::PC, reg_addr);
})
// ret
instruction!(RET, None, None, |cpu, _, _| {
    // pop pc
    let stp = cpu.registers.read_reg(Register::STP);
    println!("returning to {}",cpu.data_bus.read_byte(stp) as usize);
    cpu.registers
        .write_reg(Register::PC, cpu.data_bus.read_byte(stp) as usize);
    cpu.registers.write_reg(Register::STP, stp.wrapping_add(1));
})
);

#[macro_export]
macro_rules! instruction_set {
    ($( instruction!($name:ident, $arg1:ident, $arg2:ident, $handler:expr) )* ) => {
        pub const OPCODES: [(Instruction, &str); 72] = [$(($name, stringify!($name)), )*];
        $(
            pub const $name: Instruction = Instruction {
                arguments: (ArgumentType::$arg1, ArgumentType::$arg2),
                handler: $handler,
            };
        )*
    };
}
