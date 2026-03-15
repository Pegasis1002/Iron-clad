use crate::models::cpu::CPU;
use crate::cpu::decode::DecodeInst;

impl CPU {
    pub(crate) fn step(&mut self) -> bool {
        // 1. Check for Pending Interrupts BEFORE fetching
        // Logic: (mip & mie) != 0 AND mstatus.MIE == 1
        let mip = self.csr[0x344];
        let mie = self.csr[0x304];
        let mstatus = self.csr[0x300];
        let global_ie = (mstatus >> 3) & 1;

        if global_ie == 1 && (mip & mie) != 0 {
            self.handle_interrupt(mip & mie);
            // We handled a trap, so don't execute an instruction this cycle
            return false; 
        }

        //Fetch Instruction
        let inst = self.bus.read_word(self.pc);

        // Decode Instruction
        let decoded_instruction: DecodeInst = self.decode(inst);

        if decoded_instruction.op_code == 0x0 {
            println!("op_code = 0x0, Exiting! \n"); 
            return true;
        }

        self.execute(decoded_instruction);
        
        // MTIME Interrupt
        if self.bus.mtime >= self.bus.mtimecmp && self.bus.mtimecmp != 0 {
            // Set MTIP (Machine Timer Interrupt Pending) - Bit 7 of mip (CSR 0x344)
            // This flips the "alarm ringing" switch in the CPU
            self.csr[0x344] |= (1 << 7); 
        }

        //Increament Program counter and Cycles
        self.pc += 4;
        self.cycles += 1;

        self.csr[0xB00] = (self.cycles & 0xFFFFFFFF) as u32;
        self.csr[0xB80] = (self.cycles >> 32) as u32;

        false
    }

    fn handle_interrupt(&mut self, pending: u32) {
        // Save return address
        self.csr[0x341] = self.pc; // MEPC
        
        // Set Cause (Bit 31 = 1 for Interrupt, 7 = Timer)
        self.csr[0x342] = 0x80000007; // MCAUSE

        // Disable global interrupts in mstatus (MIE = 0, MPIE = old MIE)
        let mstatus = self.csr[0x300];
        let mie = (mstatus >> 3) & 1;
        let mut new_mstatus = (mstatus & !(1 << 3)) | (mie << 7);
        self.csr[0x300] = new_mstatus;

        // Jump to Kernel Trap Vector
        self.pc = self.csr[0x305]; // MTVEC
        println!("INTERRUPT: Timer fired! Jumping to {:#010X}", self.pc);
    }
}
