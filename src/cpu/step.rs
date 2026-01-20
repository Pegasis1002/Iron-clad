use crate::models::cpu::CPU;
use crate::cpu::decode::DecodeInst;

impl CPU {
    pub(crate) fn step(&mut self) -> bool {
        println!("---------------------------------------------\n");
        //Fetch Instruction
        let inst = self.bus.read_word(self.pc);

        // Decode Instruction
        let decoded_instruction: DecodeInst = self.decode(inst);

        if decoded_instruction.op_code == 0x0 {
            return true;
        }

        println!("Instruction at {:#X} = {:#010X}", self.pc, inst);
        println!("Decoded Instruction: {:?}", decoded_instruction);

        self.execute(decoded_instruction);
        
        //Increament Program counter
        self.pc += 4;
        println!("");
        return false;
    }
}
