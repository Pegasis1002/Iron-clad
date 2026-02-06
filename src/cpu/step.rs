use crate::models::cpu::CPU;
use crate::cpu::decode::DecodeInst;

impl CPU {
    pub(crate) fn step(&mut self) -> bool {
        //Fetch Instruction
        let inst = self.bus.read_word(self.pc);

        // Decode Instruction
        let decoded_instruction: DecodeInst = self.decode(inst);

        if decoded_instruction.op_code == 0x0 {
            println!("op_code = 0x0, Exiting! \n"); 
            return true;
        }

        self.execute(decoded_instruction);
        
        //Increament Program counter and Cycles
        self.pc += 4;
        self.cycles += 1;

        self.csr[0xB00] = (self.cycles & 0xFFFFFFFF) as u32;
        self.csr[0xB80] = (self.cycles >> 32) as u32;

        false
    }
}
