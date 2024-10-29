use solana_program::pubkey::Pubkey;
use solana_program_test::{ProgramTest, ProgramTestContext};

pub struct NebulaFixture {
    pub nebula_key: Pubkey,
    pub context: ProgramTestContext,
}

impl NebulaFixture {
    pub async fn new() -> Self {
        let mut program = ProgramTest::default();
        let nebula_key = Pubkey::new_unique();
        program.add_program("orion_nebula", nebula_key, None);
        let ctx = program.start_with_context().await;
        Self {
            nebula_key,
            context: ctx,
        }
    }
}
