use std::borrow::Borrow;

use inkwell::{
    module::Module,
    passes::{PassManager, PassManagerBuilder},
    OptimizationLevel
};


pub fn get_module_pass_manager_builder(
    optimization_level: OptimizationLevel
) -> PassManagerBuilder {
    let pmb = PassManagerBuilder::create();
    pmb.set_optimization_level(optimization_level);
    log::trace!("Module pass manager builder created.");
    pmb
}


pub fn create_module_pass_manager<'ctx>(
    optimization_level: OptimizationLevel
) -> PassManager<Module<'ctx>> {
    let pmb = get_module_pass_manager_builder(optimization_level);
    let pass_manager = PassManager::create(());
    pmb.populate_module_pass_manager(&pass_manager);
    log::trace!("Module pass manager populated.");
    pass_manager
}


pub fn run_module_pass_manager<'ctx>(
    module: &Module<'ctx>,
    optimization_level: OptimizationLevel
) -> bool {
    let pass_manager = create_module_pass_manager(optimization_level);
    log::trace!(
        "Running module pass manager on '{}'",
        module.get_name().to_string_lossy()
    );
    let result = pass_manager.run_on(module);
    log::trace!("    Result: {}", result);
    result
}


pub fn optimize_module<'ctx, M: Borrow<Module<'ctx>>>(
    module: &M,
    optimization_level: OptimizationLevel
) -> bool {
    let module = module.borrow();
    log::trace!(
        "Optimising module '{}'",
        module.get_name().to_string_lossy()
    );
    run_module_pass_manager(module, optimization_level)
}
