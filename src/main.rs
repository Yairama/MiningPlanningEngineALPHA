use amethyst::{
    prelude::*,
    utils::application_root_dir
};

struct Example;

impl EmptyState for Example{
    fn on_start(&mut self,_data: StateData<'_, ()>){
        println!("Begin!");
    }

    fn on_stop(&mut self,_data: StateData<'_, ()>){
        println!("End!");
    }

    fn update(&mut self,_data: StateData<'_, ()>)-> EmptyTrans {
        println!("Hello world");
        Trans::Quit
    }

}


fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let assets_dir = application_root_dir()?.join("assets");
    let mut game = Application::new(assets_dir, Example, ())?;
    game.run();
    Ok(())
}
