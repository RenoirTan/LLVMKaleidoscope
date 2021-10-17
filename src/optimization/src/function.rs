use inkwell::{
    module::Module,
    passes::{PassManager, PassManagerBuilder},
    values::FunctionValue,
    OptimizationLevel
};


pub fn get_function_pass_manager_builder(
    optimization_level: OptimizationLevel
) -> PassManagerBuilder {
    let pmb = PassManagerBuilder::create();
    pmb.set_optimization_level(optimization_level);
    pmb
}


pub fn create_function_pass_manager<'ctx>(
    module: &Module<'ctx>,
    optimization_level: OptimizationLevel
) -> PassManager<FunctionValue<'ctx>> {
    let pmb = get_function_pass_manager_builder(optimization_level);
    let pass_manager = PassManager::create(module);
    pmb.populate_function_pass_manager(&pass_manager);
    pass_manager
}


pub fn run_function_pass_manager<'ctx>(
    function: &FunctionValue<'ctx>,
    module: &Module<'ctx>,
    optimization_level: OptimizationLevel
) -> bool {
    let pass_manager = create_function_pass_manager(module, optimization_level);
    pass_manager.run_on(function)
}


pub fn optimize_function<'ctx, F, M>(
    function: &F,
    module: &M,
    optimization_level: OptimizationLevel
) -> bool
where
    F: AsRef<FunctionValue<'ctx>>,
    M: AsRef<Module<'ctx>>
{
    let function = function.as_ref();
    let module = module.as_ref();
    run_function_pass_manager(function, module, optimization_level)
}
