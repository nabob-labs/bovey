#[cfg(feature = "vm-internal")]
use qualifier_attr::qualifiers;
use bovey_program_runtime::declare_process_instruction;

#[cfg_attr(feature = "vm-internal", qualifiers(pub))]
const DEFAULT_COMPUTE_UNITS: u64 = 150;

declare_process_instruction!(Entrypoint, DEFAULT_COMPUTE_UNITS, |_invoke_context| {
    // Do nothing, compute budget instructions handled by the runtime
    Ok(())
});
