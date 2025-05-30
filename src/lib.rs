#![feature(ptr_sub_ptr)]
use engage::battle::BattleInfoSide;
use engage::calculator::*;
use unity::prelude::*;
use engage::gamedata::unit::*;

#[unity::hook("App", "UnitCalculator", "AddCommand")]
fn add_command_hook(calculator: &mut CalculatorManager, method_info: OptionalMethod){
    // GameCalculator is a CalculatorManager
    call_original!(calculator, method_info);

    //Doing my thing i guess idk
    let bibliophilec: &mut CalculatorCommand  = calculator.find_command("性別");   
    let bibliophile = il2cpp::instantiate_class::<GameCalculatorCommand>(bibliophilec.get_class().clone()).unwrap(); 

    // replacing get_Name function
    bibliophile.get_class_mut().get_virtual_method_mut("get_Name").map(|method| method.method_ptr = get_bibliophile_name as _); // get_move_name() returns "Mov"
    // replacing what the command grabs. This is for the unit version, which is vtable function 30
    bibliophile.get_class_mut().get_virtual_method_mut("GetImpl").map(|method| method.method_ptr = get_bibliophile_unit as _); 
    // replacing what the command grabs. This for the BattleInfoSide version.
    bibliophile.get_class_mut().get_vtable_mut()[31].method_ptr = get_bibliophile_battle_info as *mut u8; 
    
    // adding our command to the calculator manager
    calculator.add_command( bibliophile ); 

}

pub fn get_bibliophile_name(_this: &GameCalculatorCommand, _method_info: OptionalMethod) -> &'static Il2CppString {
    "Bibliophile".into()
}

// Replacing GetImpl functions with these
// GetImpl Unit Function, this will probably get called for non-battle timings
pub fn get_bibliophile_unit(_this: &GameCalculatorCommand, unit: &Unit, _method_info: OptionalMethod) -> f32 {
    let mut book_count: f32 = 0.0;
    
    for item in unit.item_list.fields.unit_items.iter() {
        let item_u = item.as_ref().unwrap();
        if item_u.item.kind == 6 {
            book_count += 1.0;
        }
    }
    
    return book_count;
}

// GetImpl(BattleInfoSide) This will probably get called during battle timings (2-18)
pub fn get_bibliophile_battle_info(_this: &GameCalculatorCommand, side: &BattleInfoSide, _method_info: OptionalMethod) -> f32 {
    let mut book_count: f32 = 0.0;

    for item in side.unit.unwrap().item_list.fields.unit_items.iter() {
        let item_u = item.as_ref().unwrap();
        if item_u.item.kind == 6 {
            book_count += 1.0;
        }
    }

    return book_count;
}

#[skyline::main(name = "bibliophile")]
pub fn main() {
    std::panic::set_hook(Box::new(|info| {
        let location = info.location().unwrap();
        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => {
                match info.payload().downcast_ref::<String>() {
                    Some(s) => &s[..],
                    None => "Box<Any>",
                }
            },
        };

        let err_msg = format!(
            "SkillCommand plugin has panicked at '{}' with the following message:\n{}\0",
            location,
            msg
        );

        skyline::error::show_error(
            42069,
            "SkillCommand plugin has panicked! Please open the details and send a screenshot to the developer, then close the game.\n\0",
            err_msg.as_str(),
        );
    }));

    skyline::install_hooks!(add_command_hook);
}