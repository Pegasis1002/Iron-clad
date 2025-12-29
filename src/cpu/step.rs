use crate::models::cpu::CPU;

impl CPU {
    pub(crate) fn step(&mut self) {
        //Fetch Instruction
        let op_code = self.bus.read_word(self.pc);

        println!("Instruction at {:#X} = {:#010X}", self.pc, op_code);
        // Decode OP_code
        self.decode(op_code);

        //Increament Program counter
        self.pc += 4;
    }
}
