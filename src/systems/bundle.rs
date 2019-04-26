use amethyst::{
    core::bundle::SystemBundle,
    ecs::DispatcherBuilder,
    error::Error,
};

/// Bundle containing all `System`s that should be running independent of any `State`.
pub struct CoreSystemsBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for CoreSystemsBundle {
    fn build(self, _dispatcher: &mut DispatcherBuilder) -> Result<(), Error> {
        // TODO: actually add core systems

        Ok(())
    }
}