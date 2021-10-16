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
    pmb
}


pub fn create_module_pass_manager<'ctx>(
    optimization_level: OptimizationLevel
) -> PassManager<Module<'ctx>> {
    let pmb = get_module_pass_manager_builder(optimization_level);
    let pass_manager = PassManager::create(());
    pmb.populate_module_pass_manager(&pass_manager);
    pass_manager
}


pub fn run_module_pass_manager<'ctx>(
    module: &Module<'ctx>,
    optimization_level: OptimizationLevel
) -> bool {
    let pass_manager = create_module_pass_manager(optimization_level);
    pass_manager.run_on(module)
}


pub fn optimize_module<'ctx, T: AsRef<Module<'ctx>>>(
    thing: &T,
    optimization_level: OptimizationLevel
) -> bool {
    let module = thing.as_ref();
    run_module_pass_manager(module, optimization_level)
}
