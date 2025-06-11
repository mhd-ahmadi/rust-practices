struct Computer {
    cpu: String,
    memory: u32,
    hard_drive_capacity: u32
}

impl Computer {
    fn new(cpu: String, memory: u32, hard_drive_capacity: u32) -> Self {
        Self {
            cpu, memory, hard_drive_capacity
        }
    }

    fn upgrade_cpu(&mut self, new_cpu: String) -> &mut Self {
        self.cpu = new_cpu;
        self
    }

    fn update_memory(&mut self, new_memory: u32) -> &mut Self {
        self.memory = new_memory;
        self
    }

    fn upgrade_hard_drive_capacity(&mut self, new_capacity: u32) -> &mut Self {
        self.hard_drive_capacity = new_capacity;
        self
    }
}

fn main() {
    let mut pc = Computer::new(String::from("Core i7 12700"), 32, 1);

    pc.upgrade_cpu(String::from("Core i9 14900"))
      .update_memory(64)
      .upgrade_hard_drive_capacity(2);
}