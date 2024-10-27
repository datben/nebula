use solana_program::pubkey::Pubkey;
use solana_program_test::{ProgramTest, ProgramTestContext};

pub struct NebulaFixture {
    pub native_key: Pubkey,
    pub nebula_key: Pubkey,
    pub context: ProgramTestContext,
}

impl NebulaFixture {
    pub async fn new() -> Self {
        let mut program = ProgramTest::default();
        let native_key = Pubkey::new_unique();
        let nebula_key = Pubkey::new_unique();
        program.add_program("orion_native", native_key, None);
        program.add_program("orion_nebula", nebula_key, None);
        program.prefer_bpf(true);
        let ctx = program.start_with_context().await;
        Self {
            nebula_key,
            native_key,
            context: ctx,
        }
    }
}
